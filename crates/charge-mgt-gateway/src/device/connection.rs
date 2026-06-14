//! 单条 WebSocket 连接的消息处理
//!
//! 解析 OCPP Call 消息，转发至 Kafka，并根据 action 类型决定
//! 立即响应或等待云端异步响应。

use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::cloud::{
    CloudMessage, CloudMessageInput, ConnectionManager, ConnectionMeta, KafkaProducer,
};
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;

type Call = ocpp_1_6::envelope::Call;
type CallResult = ocpp_1_6::envelope::CallResult;
type CallError = ocpp_1_6::envelope::CallError;

/// 判断该 action 是否需要等待云端响应（而非网关立即回复）
fn requires_cloud_response(action: &str) -> bool {
    matches!(
        action,
        "BootNotification" | "Authorize" | "StartTransaction" | "StopTransaction"
    )
}

/// 代表一条充电桩 WebSocket 连接及其会话状态
pub struct Connection {
    /// 连接唯一 ID（BootNotification 前作为临时 charge_point_id）
    pub id: String,
    /// 充电桩 TCP 远端地址
    pub addr: SocketAddr,
    /// 充电桩厂商（BootNotification 后填充）
    charge_point_vendor: Option<String>,
    /// 充电桩型号（BootNotification 后填充）
    charge_point_model: Option<String>,
    /// 充电桩序列号/ID（BootNotification 后填充）
    charge_point_id: Option<String>,
    /// 全局连接管理器
    connection_manager: Arc<ConnectionManager>,
    /// Kafka 生产者，用于上行消息发布
    kafka_producer: Arc<KafkaProducer>,
    /// 云端响应回传通道
    response_channel: Arc<dyn ResponseChannel>,
    /// 所属网关 ID
    gateway_id: String,
    /// 所属网关 IP
    gateway_host: String,
    /// 响应发送通道，将 OCPP 响应写入 WebSocket 写任务
    response_tx: mpsc::UnboundedSender<String>,
}

impl Connection {
    /// 创建新连接，初始时 charge_point_id 使用连接 UUID
    pub fn new(
        addr: SocketAddr,
        connection_manager: Arc<ConnectionManager>,
        kafka_producer: Arc<KafkaProducer>,
        response_channel: Arc<dyn ResponseChannel>,
        gateway_id: String,
        gateway_host: String,
        response_tx: mpsc::UnboundedSender<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            addr,
            charge_point_vendor: None,
            charge_point_model: None,
            charge_point_id: None,
            connection_manager,
            kafka_producer,
            response_channel,
            gateway_id,
            gateway_host,
            response_tx,
        }
    }

    fn current_charge_point_id(&self) -> String {
        self.charge_point_id
            .clone()
            .unwrap_or_else(|| self.id.clone())
    }

    /// 处理充电桩发来的 OCPP Call 消息：解析 → 转发 Kafka → 响应
    pub async fn handle_message(&mut self, text: &str) -> Result<()> {
        info!("原始消息（长度={}）: {}", text.len(), text);
        let call: Call = match serde_json::from_str(text) {
            Ok(c) => c,
            Err(e) => {
                warn!("OCPP Call 消息解析失败: 长度={}, 错误={}", text.len(), e);
                return Err(GatewayError::Codec(format!("无效的 OCPP 消息: {}", e)));
            }
        };

        info!(
            "收到 OCPP Call: 动作={}, uniqueId={}",
            call.action, call.unique_id
        );

        self.process_meta(&call.action, &call.payload).await;

        let cloud_msg = self.build_cloud_message(&call);
        match self.kafka_producer.send(&cloud_msg).await {
            Ok(()) => {
                info!(
                    "[KAFKA] 发送成功: 动作={}, uniqueId={}",
                    call.action, call.unique_id
                );
            }
            Err(e) => {
                error!(
                    "[KAFKA] 发送失败: 动作={}, uniqueId={}, 错误={}",
                    call.action, call.unique_id, e
                );
            }
        }

        let action = call.action.as_str();
        let unique_id = call.unique_id.clone();
        let cp_id = self.current_charge_point_id();

        if requires_cloud_response(action) {
            info!("待响应请求已分发: 动作={}, uniqueId={}", action, unique_id);

            self.response_channel.dispatch_pending_request(
                unique_id,
                cp_id,
                action.to_string(),
                self.response_tx.clone(),
            );
        } else {
            let response = self.handle_immediate(action, &unique_id)?;
            self.response_tx.send(response).ok();
        }

        Ok(())
    }

    // 从消息 payload 中提取并更新连接元数据（厂商、序列号等）
    async fn process_meta(&mut self, action: &str, payload: &Value) {
        match action {
            "BootNotification" => self.process_boot_notification_meta(payload).await,
            "Authorize" => self.process_authorize_meta(payload),
            "StartTransaction" => self.process_start_transaction_meta(payload),
            "StopTransaction" => self.process_stop_transaction_meta(payload),
            _ => {}
        }
    }

    async fn process_boot_notification_meta(&mut self, payload: &Value) {
        let vendor = payload
            .get("chargePointVendor")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let model = payload
            .get("chargePointModel")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let serial = payload
            .get("chargeBoxSerialNumber")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        info!(
            "BootNotification 元数据: 厂商={}, 型号={}, 序列号={:?}",
            vendor, model, serial
        );

        self.charge_point_vendor = Some(vendor.clone());
        self.charge_point_model = Some(model.clone());

        let old_id = self.current_charge_point_id();
        self.charge_point_id = serial.clone();
        let new_id = self.current_charge_point_id();

        let meta = ConnectionMeta {
            charge_point_id: new_id.clone(),
            vendor: vendor.clone(),
            protocol_version: "OCPP-1.6".to_string(),
            connected_at: chrono::Utc::now(),
            response_tx: self.response_tx.clone(),
        };

        if old_id != new_id {
            self.connection_manager.remove_connection(&old_id).await;
        }
        self.connection_manager
            .add_connection(new_id.clone(), meta)
            .await;
        info!("充电桩 {} 已注册到连接管理器", new_id);
    }

    fn process_authorize_meta(&mut self, payload: &Value) {
        let id_tag = payload
            .get("idTag")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        info!("Authorize 元数据: idTag={}", id_tag);
    }

    fn process_start_transaction_meta(&mut self, payload: &Value) {
        let connector_id = payload
            .get("connectorId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let id_tag = payload
            .get("idTag")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        info!(
            "StartTransaction 元数据: 连接器={}, idTag={}",
            connector_id, id_tag
        );
    }

    fn process_stop_transaction_meta(&mut self, payload: &Value) {
        let transaction_id = payload
            .get("transactionId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        info!("StopTransaction 元数据: 交易ID={}", transaction_id);
    }

    /// 对无需云端参与的消息（如 Heartbeat）直接生成 CallResult 响应
    fn handle_immediate(&self, action: &str, unique_id: &str) -> Result<String> {
        match action {
            "Heartbeat" => {
                let response_payload = serde_json::json!({
                    "currentTime": chrono::Utc::now().to_rfc3339()
                });
                let response = CallResult::new(unique_id, response_payload);
                Ok(serde_json::to_string(&response)
                    .map_err(|e| GatewayError::Codec(e.to_string()))?)
            }
            "StatusNotification" => {
                let response = CallResult::new(unique_id, serde_json::json!({}));
                Ok(serde_json::to_string(&response)
                    .map_err(|e| GatewayError::Codec(e.to_string()))?)
            }
            "MeterValues" => {
                let response = CallResult::new(unique_id, serde_json::json!({}));
                Ok(serde_json::to_string(&response)
                    .map_err(|e| GatewayError::Codec(e.to_string()))?)
            }
            _ => {
                let call_error = CallError::new(
                    unique_id,
                    "NotImplemented",
                    &format!("不支持的操作: {}", action),
                );
                Ok(serde_json::to_string(&call_error)
                    .map_err(|e| GatewayError::Codec(e.to_string()))?)
            }
        }
    }

    /// 将 OCPP Call 封装为 CloudMessage 用于 Kafka 发布
    pub fn build_cloud_message(&self, call: &Call) -> CloudMessage {
        let input = CloudMessageInput {
            gateway_id: self.gateway_id.clone(),
            gateway_ip: self.gateway_host.clone(),
            vendor: self
                .charge_point_vendor
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            charge_point_id: self.current_charge_point_id(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "Call".to_string(),
            action: call.action.clone(),
            unique_id: call.unique_id.clone(),
        };
        CloudMessage::new(input, call.payload.clone())
    }

    /// 构造 OCPP CallError 响应 JSON
    pub fn create_call_error(&self, unique_id: &str, error_code: &str, error_desc: &str) -> String {
        let error = CallError::new(unique_id, error_code, error_desc);
        serde_json::to_string(&error)
            .unwrap_or_else(|_| r#"[4,"","GenericError","Error",null]"#.to_string())
    }

    /// 连接断开时从连接管理器中注销
    pub async fn on_disconnect(&mut self) {
        let cp_id = self.current_charge_point_id();
        info!("充电桩断开连接: {}", cp_id);
        self.connection_manager.remove_connection(&cp_id).await;
    }
}
