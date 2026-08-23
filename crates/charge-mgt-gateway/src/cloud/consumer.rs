//! Kafka 消费者与连接管理
//!
//! - `ConnectionManager`：维护在线充电桩连接，支持下行消息推送
//! - `KafkaConsumer`：订阅云端下行消息，转换为 OCPP 格式回传充电桩

use futures_util::StreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{error, info, warn};

use charge_mgt_common::ocpp16::CloudMessage;
use crate::config::KafkaConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::{MessageDirection, PendingRequestTracker};

use ocpp_1_6::envelope::{Call, CallError, CallResult};

/// 单个充电桩连接的元数据，包含下行消息发送通道
#[derive(Clone)]
pub struct ConnectionMeta {
    /// 充电桩 ID
    pub charge_point_id: String,
    /// 充电桩厂商
    pub vendor: String,
    /// OCPP 协议版本
    pub protocol_version: String,
    /// 连接建立时间
    pub connected_at: chrono::DateTime<chrono::Utc>,
    /// 下行消息发送通道，用于向充电桩 WebSocket 写回响应
    pub response_tx: mpsc::UnboundedSender<String>,
}

/// 充电桩连接注册表，以 charge_point_id 为键
pub struct ConnectionManager {
    /// 以 charge_point_id 为键的在线连接表
    connections: Arc<RwLock<std::collections::HashMap<String, ConnectionMeta>>>,
}

impl ConnectionManager {
    /// 创建空的连接管理器
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 注册新连接
    pub async fn add_connection(&self, charge_point_id: String, meta: ConnectionMeta) {
        let mut connections = self.connections.write().await;
        connections.insert(charge_point_id, meta);
    }

    /// 更新已有连接的元数据
    pub async fn update_connection(&self, charge_point_id: &str, meta: ConnectionMeta) {
        let mut connections = self.connections.write().await;
        connections.insert(charge_point_id.to_string(), meta);
    }

    /// 移除断开的连接
    pub async fn remove_connection(&self, charge_point_id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(charge_point_id);
    }

    /// 列出所有在线充电桩 ID
    pub async fn list_charge_points(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    /// 查询指定充电桩的连接信息
    pub async fn get_connection_info(&self, charge_point_id: &str) -> Option<ConnectionMeta> {
        let connections = self.connections.read().await;
        connections.get(charge_point_id).cloned()
    }

    /// 向指定充电桩发送 OCPP 消息，连接不存在或通道关闭时返回 false
    pub async fn send_to_charge_point(&self, charge_point_id: &str, message: String) -> bool {
        match self.get_connection_info(charge_point_id).await {
            Some(meta) => {
                if meta.response_tx.send(message).is_ok() {
                    info!("消息已发送至充电桩 {}", charge_point_id);
                    true
                } else {
                    warn!("发送至充电桩 {} 失败（通道已关闭）", charge_point_id);
                    false
                }
            }
            None => {
                warn!("充电桩 {} 未连接，消息已丢弃", charge_point_id);
                false
            }
        }
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Kafka 消费者，根据响应通道模式订阅不同主题
pub struct KafkaConsumer {
    /// rdkafka 流式消费者
    consumer: StreamConsumer,
    /// 充电桩连接管理器，用于下行消息推送
    connection_manager: Arc<ConnectionManager>,
    /// 待响应请求跟踪器（Kafka 响应通道模式）
    pending_tracker: Option<Arc<PendingRequestTracker>>,
}

impl KafkaConsumer {
    /// Redis 模式：订阅 cmd 主题，接收云端下发的 Call 命令
    pub fn new_redis_mode(
        config: &KafkaConfig,
        gateway_id: &str,
        connection_manager: Arc<ConnectionManager>,
    ) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", format!("gateway-{}-cmd-consumer", gateway_id))
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "5000")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("创建消费者失败: {}", e)))?;

        let cmd_topic = CloudMessage::cmd_topic(&config.topic_prefix, gateway_id);
        consumer
            .subscribe(&[&cmd_topic])
            .map_err(|e| GatewayError::Kafka(format!("订阅主题失败: {}", e)))?;

        info!("Kafka 消费者（Redis 模式）已订阅: {}", cmd_topic);

        Ok(Self {
            consumer,
            connection_manager,
            pending_tracker: None,
        })
    }

    /// Kafka 模式：订阅 resp 主题，接收云端 CallResult/CallError 响应
    pub fn new_kafka_mode(
        config: &KafkaConfig,
        gateway_id: &str,
        connection_manager: Arc<ConnectionManager>,
        pending_tracker: Arc<PendingRequestTracker>,
    ) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", format!("gateway-{}-resp-consumer", gateway_id))
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set("session.timeout.ms", "10000")
            .set("socket.connection.setup.timeout.ms", "5000")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("创建消费者失败: {}", e)))?;

        let resp_topic = CloudMessage::resp_topic(&config.topic_prefix, gateway_id);
        consumer
            .subscribe(&[&resp_topic])
            .map_err(|e| GatewayError::Kafka(format!("订阅主题失败: {}", e)))?;

        info!("Kafka 消费者（Kafka 模式）已订阅: {}", resp_topic);

        Ok(Self {
            consumer,
            connection_manager,
            pending_tracker: Some(pending_tracker),
        })
    }

    /// 持续消费 Kafka 消息并分发处理
    pub async fn run(&self) {
        info!("Kafka 消费者已启动");
        let stream = self.consumer.stream();

        futures_util::pin_mut!(stream);

        while let Some(msg_result) = stream.next().await {
            match msg_result {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        match serde_json::from_slice::<CloudMessage>(payload) {
                            Ok(cloud_msg) => {
                                self.handle_downstream_message(cloud_msg).await;
                            }
                            Err(e) => {
                                warn!("云端消息解析失败: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Kafka 错误: {}", e);
                }
            }
        }
    }

    // 根据 message_type 分发到对应的 OCPP 消息构建逻辑
    async fn handle_downstream_message(&self, msg: CloudMessage) {
        let charge_point_id = &msg.charge_point_id.as_deref().unwrap_or("");

        info!(
            "收到下行消息: 类型={}, 动作={}, 充电桩={}",
            msg.message_type.as_deref().unwrap_or(""), msg.action.as_deref().unwrap_or(""), charge_point_id
        );

        match msg.message_type.as_deref() {
            Some("CallResult") => self.handle_call_result(&msg, charge_point_id).await,
            Some("CallError") => self.handle_call_error(&msg, charge_point_id).await,
            Some("Call") => self.handle_call(&msg, charge_point_id).await,
            _ => warn!("未知消息类型: {}", msg.message_type.as_deref().unwrap_or("")),
        }
    }

    // 匹配 pending tracker 中的 uniqueId，将响应路由到正确的 WebSocket 连接
    async fn handle_call_result(&self, msg: &CloudMessage, charge_point_id: &str) {
        let tracker = match &self.pending_tracker {
            Some(t) => t,
            None => {
                warn!(
                    "收到 CallResult 但无待响应跟踪器（Redis 模式不应通过 Kafka 接收 CallResult）"
                );
                self.connection_manager
                    .send_to_charge_point(
                        charge_point_id,
                        build_ocpp_call_result(&msg.unique_id.as_deref().unwrap_or(""), msg.payload.clone().unwrap_or(serde_json::Value::Null)),
                    )
                    .await;
                return;
            }
        };

        let pending = tracker.remove(&msg.unique_id.as_deref().unwrap_or("")).await;
        match pending {
            Some(request) => {
                info!(
                    "云端 CallResult 已匹配待响应请求: uniqueId={}, 动作={}",
                    request.unique_id, request.action
                );
                let call_result_json = build_ocpp_call_result(&msg.unique_id.as_deref().unwrap_or(""), msg.payload.clone().unwrap_or(serde_json::Value::Null));
                request.response_tx.send(call_result_json).ok();
            }
            None => {
                warn!(
                    "CallResult 无匹配待响应请求 uniqueId={}，直接转发至充电桩",
                    msg.unique_id.as_deref().unwrap_or("")
                );
                self.connection_manager
                    .send_to_charge_point(
                        charge_point_id,
                        build_ocpp_call_result(&msg.unique_id.as_deref().unwrap_or(""), msg.payload.clone().unwrap_or(serde_json::Value::Null)),
                    )
                    .await;
            }
        }
    }

    async fn handle_call_error(&self, msg: &CloudMessage, charge_point_id: &str) {
        let tracker = match &self.pending_tracker {
            Some(t) => t,
            None => {
                let error_json = build_ocpp_call_error(
                    &msg.unique_id.as_deref().unwrap_or(""),
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                self.connection_manager
                    .send_to_charge_point(charge_point_id, error_json)
                    .await;
                return;
            }
        };

        let pending = tracker.remove(&msg.unique_id.as_deref().unwrap_or("")).await;
        match pending {
            Some(request) => {
                info!(
                    "云端 CallError 已匹配待响应请求: uniqueId={}, 动作={}",
                    request.unique_id, request.action
                );
                let error_json = build_ocpp_call_error(
                    &msg.unique_id.as_deref().unwrap_or(""),
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                request.response_tx.send(error_json).ok();
            }
            None => {
                let error_json = build_ocpp_call_error(
                    &msg.unique_id.as_deref().unwrap_or(""),
                    msg.error_code.as_deref().unwrap_or("InternalError"),
                    msg.error_description.as_deref().unwrap_or("Unknown error"),
                );
                self.connection_manager
                    .send_to_charge_point(charge_point_id, error_json)
                    .await;
            }
        }
    }

    // 云端下发的 Call 命令：转发至充电桩并注册下行 pending 请求
    async fn handle_call(&self, msg: &CloudMessage, charge_point_id: &str) {
        let call_json = build_ocpp_call(&msg.action.as_deref().unwrap_or(""), &msg.unique_id.as_deref().unwrap_or(""), msg.payload.clone().unwrap_or(serde_json::Value::Null));

        if self
            .connection_manager
            .send_to_charge_point(charge_point_id, call_json)
            .await
        {
            if let Some(tracker) = &self.pending_tracker {
                let meta = self
                    .connection_manager
                    .get_connection_info(charge_point_id)
                    .await;
                if let Some(meta) = meta {
                    tracker
                        .register(crate::response_channel::PendingRequest {
                            unique_id: msg.unique_id.clone().unwrap_or_default(),
                            charge_point_id: charge_point_id.to_string(),
                            action: msg.action.clone().unwrap_or_default(),
                            direction: MessageDirection::Downstream,
                            created_at: std::time::Instant::now(),
                            response_tx: meta.response_tx,
                        })
                        .await;
                }
            }
        }
    }
}

/// 将云端消息转换为 OCPP Call JSON
fn build_ocpp_call(action: &str, unique_id: &str, payload: serde_json::Value) -> String {
    let call = Call::new(action, unique_id, payload);
    serde_json::to_string(&call).unwrap_or_default()
}

/// 将云端响应转换为 OCPP CallResult JSON
fn build_ocpp_call_result(unique_id: &str, payload: serde_json::Value) -> String {
    let call_result = CallResult::new(unique_id, payload);
    serde_json::to_string(&call_result).unwrap_or_default()
}

/// 将云端错误转换为 OCPP CallError JSON
fn build_ocpp_call_error(unique_id: &str, error_code: &str, error_description: &str) -> String {
    let call_error = CallError::new(unique_id, error_code, error_description);
    serde_json::to_string(&call_error).unwrap_or_default()
}
