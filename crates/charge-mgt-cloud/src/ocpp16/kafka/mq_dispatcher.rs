//! OCPP-J 出站响应分发器。
//!
//! CSMS 主动下发 CALL 报文后，桩通过 `<prefix>.resp.<gateway_id>` topic 回送
//! CALLRESULT / CALLERROR。本模块维护**一个**共享的 `StreamConsumer`，常驻订阅所有
//! `*.resp.*` topic；后台 tokio 任务把响应按 `unique_id` 分发给对应的等待者。
//!
//! 调用方通过 [`MqDispatcher::await_response`] 拿到一个一次性 future：

use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};

use crate::error::AppError;

type PendingMap = DashMap<String, oneshot::Sender<ResponsePayload>>;

/// 响应载荷：原始 Kafka 字节 + 携带的 `unique_id`（用于失败回放/日志）。
#[derive(Debug)]
pub struct ResponsePayload {
    pub bytes: Vec<u8>,
}

/// 共享分发器：`Arc<MqDispatcher>`，内部包含后台消费任务。
pub struct MqDispatcher {
    pending: Arc<PendingMap>,
    consumer_group: String,
    consumer: Arc<StreamConsumer>,
}

impl MqDispatcher {
    /// 构造并 spawn 后台任务。
    ///
    /// - `brokers` / `consumer_group`：Kafka 连接与 group.id
    /// - `topic_prefix`：自动订阅所有 `{prefix}.resp.*` topic
    /// - `start_topics`：可选的初始订阅列表（来自配置 `resp_topics`）；为空则启动时自动发现
    pub fn spawn(
        brokers: &str,
        consumer_group: &str,
        topic_prefix: &str,
        start_topics: Vec<String>,
    ) -> Result<Arc<Self>, AppError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", consumer_group)
            .set("bootstrap.servers", brokers)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .create()
            .map_err(|e| AppError::Internal(format!("创建 Kafka resp 消费者失败：{e}")))?;

        let topics = if !start_topics.is_empty() {
            start_topics
        } else {
            Self::discover_resp_topics(&consumer, topic_prefix)
        };

        if topics.is_empty() {
            warn!("无可用 resp 话题，分发器空闲（Gateway 启动后重启 cloud）");
        } else {
            let refs: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
            consumer
                .subscribe(&refs)
                .map_err(|e| AppError::Internal(format!("订阅 resp 话题失败：{e}")))?;
            info!(
                consumer_group = %consumer_group,
                topics = ?topics,
                "MQ resp 分发器已订阅 {} 个话题",
                topics.len()
            );
        }

        let me = Arc::new(Self {
            pending: Arc::new(DashMap::new()),
            consumer_group: consumer_group.to_string(),
            consumer: Arc::new(consumer),
        });

        let pending = me.pending.clone();
        let consumer = me.consumer.clone();
        let prefix = topic_prefix.to_string();
        tokio::spawn(async move {
            Self::run_loop(consumer, pending, prefix).await;
        });

        Ok(me)
    }

    /// 注册一个等待者：返回 oneshot receiver，调用方 `await` 等响应。
    ///
    /// `unique_id` 必须与发出去的 CALL 的 unique_id 一致；响应到达后台循环时
    /// 会按 `unique_id` 找到对应 sender 并发送。
    pub fn register(&self, unique_id: String) -> oneshot::Receiver<ResponsePayload> {
        let (tx, rx) = oneshot::channel();
        self.pending.insert(unique_id, tx);
        rx
    }

    /// 便捷组合：注册 + 超时等待。
    pub async fn await_response(
        &self,
        unique_id: &str,
        timeout: Duration,
    ) -> Result<ResponsePayload, AppError> {
        let rx = self.register(unique_id.to_string());
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(payload)) => Ok(payload),
            Ok(Err(_canceled)) => Err(AppError::Internal(format!(
                "MQ resp 等待者被取消：unique_id={unique_id}"
            ))),
            Err(_elapsed) => {
                self.pending.remove(unique_id);
                Err(AppError::OCPP_1_6_ERROR {
                    action: "MQ_RESP_TIMEOUT".into(),
                    detail: format!("等待 OCPP 响应超时（{timeout:?}）：unique_id={unique_id}"),
                })
            }
        }
    }

    async fn run_loop(
        consumer: Arc<StreamConsumer>,
        pending: Arc<PendingMap>,
        topic_prefix: String,
    ) {
        let pattern = format!("{topic_prefix}.resp.");
        let mut stream = consumer.stream();
        let mut subscribed_count = consumer.subscription().map(|t| t.count()).unwrap_or(0);

        loop {
            match stream.next().await {
                Some(Ok(borrowed)) => {
                    let Some(payload) = borrowed.payload() else {
                        continue;
                    };
                    // unique_id 是 Kafka record 的 key（producer 端约定）
                    let unique_id = borrowed
                        .key()
                        .and_then(|k| std::str::from_utf8(k).ok())
                        .map(|s| s.to_string());

                    let Some(unique_id) = unique_id else {
                        warn!("MQ resp 消息缺少 key，跳过");
                        continue;
                    };

                    let bytes = payload.to_vec();
                    debug!(unique_id = %unique_id, bytes_len = bytes.len(), "MQ resp 收到响应");

                    if let Some((_, tx)) = pending.remove(&unique_id) {
                        // oneshot 失败 = 接收端已 drop（多半是超时清理过了）
                        let _ = tx.send(ResponsePayload { bytes });
                    } else {
                        debug!(
                            unique_id = %unique_id,
                            "MQ resp 到达但无等待者（可能超时后到达，丢弃）"
                        );
                    }
                }
                Some(Err(e)) => {
                    error!(error = %e, "MQ resp 消费错误");
                }
                None => {
                    warn!("MQ resp 流已结束");
                    break;
                }
            }

            // 定期重发现新增 resp topic（与 consumer.rs 同样的策略）
            if let Ok(metadata) = consumer.fetch_metadata(None, Duration::from_secs(5)) {
                let discovered: Vec<String> = metadata
                    .topics()
                    .iter()
                    .map(|t| t.name().to_string())
                    .filter(|n| n.starts_with(&pattern))
                    .collect();
                if discovered.len() > subscribed_count {
                    info!(
                        "MQ resp 重发现：{} -> {} 个话题",
                        subscribed_count,
                        discovered.len()
                    );
                    let refs: Vec<&str> = discovered.iter().map(|s| s.as_str()).collect();
                    if consumer.subscribe(&refs).is_ok() {
                        subscribed_count = discovered.len();
                        stream = consumer.stream();
                    }
                }
            }
        }
        warn!("MQ resp 分发后台任务退出");
    }

    fn discover_resp_topics(consumer: &StreamConsumer, prefix: &str) -> Vec<String> {
        let pattern = format!("{prefix}.resp.");
        let metadata = match consumer.fetch_metadata(None, Duration::from_secs(10)) {
            Ok(m) => m,
            Err(e) => {
                warn!("拉取 Kafka metadata 失败：{e}");
                return Vec::new();
            }
        };
        metadata
            .topics()
            .iter()
            .map(|t| t.name().to_string())
            .filter(|n| n.starts_with(&pattern))
            .collect()
    }
}
