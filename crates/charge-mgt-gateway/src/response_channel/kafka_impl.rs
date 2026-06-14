//! Kafka 响应通道实现
//!
//! 通过 PendingRequestTracker 跟踪待响应请求，
//! 由 KafkaConsumer 在收到响应主题消息时完成 uniqueId 匹配。

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};

use crate::config::KafkaConfig;
use crate::response_channel::ResponseChannel;

use ocpp_1_6::envelope::CallError;

/// 待响应请求的方向：上行（CP→云）或下行（云→CP）
#[derive(Debug, Clone)]
pub enum MessageDirection {
    Upstream,
    Downstream,
}

/// 一条待匹配响应的请求记录
#[derive(Debug, Clone)]
pub struct PendingRequest {
    /// OCPP 消息唯一 ID，用于响应匹配
    pub unique_id: String,
    /// 关联的充电桩 ID
    pub charge_point_id: String,
    /// OCPP action 名称
    pub action: String,
    /// 请求方向：上行（CP→云）或下行（云→CP）
    pub direction: MessageDirection,
    /// 请求注册时间，用于超时判断
    pub created_at: Instant,
    /// 响应回传通道，匹配成功后通过此通道发送 OCPP 响应
    pub response_tx: mpsc::UnboundedSender<String>,
}

/// 按 uniqueId 索引的待响应请求注册表，支持超时清理
pub struct PendingRequestTracker {
    /// 以 unique_id 为键的待响应请求表
    pending: Arc<RwLock<std::collections::HashMap<String, PendingRequest>>>,
    /// 响应超时时间（秒）
    timeout_secs: u64,
}

impl PendingRequestTracker {
    /// 创建跟踪器，指定响应超时秒数
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            pending: Arc::new(RwLock::new(std::collections::HashMap::new())),
            timeout_secs,
        }
    }

    /// 注册待响应请求
    pub async fn register(&self, request: PendingRequest) {
        let mut pending = self.pending.write().await;
        if pending.contains_key(&request.unique_id) {
            warn!(
                "重复的 uniqueId {}，将覆盖之前的待响应请求",
                request.unique_id
            );
        }
        info!(
            "待响应请求已注册: uniqueId={}, 动作={}",
            request.unique_id, request.action
        );
        pending.insert(request.unique_id.clone(), request);
    }

    /// 响应到达时移除并返回对应的待响应请求
    pub async fn remove(&self, unique_id: &str) -> Option<PendingRequest> {
        let mut pending = self.pending.write().await;
        pending.remove(unique_id)
    }

    /// 充电桩断开时清理其所有待响应请求
    pub async fn remove_by_charge_point(&self, charge_point_id: &str) -> Vec<PendingRequest> {
        let mut pending = self.pending.write().await;
        let keys_to_remove: Vec<String> = pending
            .values()
            .filter(|r| r.charge_point_id == charge_point_id)
            .map(|r| r.unique_id.clone())
            .collect();

        let removed: Vec<PendingRequest> = keys_to_remove
            .iter()
            .filter_map(|k| pending.remove(k))
            .collect();

        if !removed.is_empty() {
            info!(
                "已清理充电桩 {} 的 {} 条待响应请求",
                charge_point_id,
                removed.len()
            );
        }
        removed
    }

    /// 启动后台任务，每 5 秒扫描并清理超时请求
    pub fn start_timeout_eviction(self: &Arc<Self>) {
        let pending = self.pending.clone();
        let timeout_secs = self.timeout_secs;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                let mut pending_guard = pending.write().await;
                let now = Instant::now();
                let timeout = std::time::Duration::from_secs(timeout_secs);

                let timed_out_keys: Vec<String> = pending_guard
                    .values()
                    .filter(|r| now.duration_since(r.created_at) > timeout)
                    .map(|r| r.unique_id.clone())
                    .collect();

                for key in timed_out_keys {
                    if let Some(request) = pending_guard.remove(&key) {
                        warn!(
                            "请求超时: uniqueId={}, 动作={}, 充电桩={}",
                            request.unique_id, request.action, request.charge_point_id
                        );

                        match request.direction {
                            MessageDirection::Upstream => {
                                let call_error = CallError::new(
                                    &request.unique_id,
                                    "InternalError",
                                    "云平台响应超时",
                                );
                                let error_json =
                                    serde_json::to_string(&call_error).unwrap_or_default();
                                request.response_tx.send(error_json).ok();
                            }
                            MessageDirection::Downstream => {
                                warn!(
                                    "云端命令等待充电桩响应超时: 动作={}, 充电桩={}",
                                    request.action, request.charge_point_id
                                );
                            }
                        }
                    }
                }
            }
        });
    }
}

/// 基于 Kafka 响应主题 + PendingRequestTracker 的响应通道
pub struct KafkaResponseChannel {
    /// 待响应请求跟踪器
    pending_tracker: Arc<PendingRequestTracker>,
}

impl KafkaResponseChannel {
    /// 创建 Kafka 响应通道并启动超时清理任务
    pub fn new(config: &KafkaConfig) -> Arc<Self> {
        let tracker = Arc::new(PendingRequestTracker::new(config.response_timeout_secs));
        tracker.start_timeout_eviction();
        Arc::new(Self {
            pending_tracker: tracker,
        })
    }

    /// 获取待响应请求跟踪器，供 KafkaConsumer 匹配响应使用
    pub fn pending_tracker(&self) -> Arc<PendingRequestTracker> {
        self.pending_tracker.clone()
    }
}

impl ResponseChannel for KafkaResponseChannel {
    fn dispatch_pending_request(
        &self,
        unique_id: String,
        charge_point_id: String,
        action: String,
        response_tx: mpsc::UnboundedSender<String>,
    ) {
        // 异步注册到跟踪器，由 KafkaConsumer 后台任务完成响应匹配
        let tracker = self.pending_tracker.clone();
        tokio::spawn(async move {
            tracker
                .register(PendingRequest {
                    unique_id,
                    charge_point_id,
                    action,
                    direction: MessageDirection::Upstream,
                    created_at: Instant::now(),
                    response_tx,
                })
                .await;
        });
    }
}
