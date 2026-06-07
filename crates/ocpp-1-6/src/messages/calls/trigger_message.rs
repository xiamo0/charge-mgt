//! TriggerMessage 消息及处理器
//!
//! 用于远程触发充电点发送特定消息（如 Heartbeat、MeterValues 等）。

use super::super::confs::trigger_message_conf::TriggerMessageConfirmation;
use crate::common::status::MessageTrigger;
use serde::{Deserialize, Serialize};

/// TriggerMessage 请求，指定要触发的消息类型和可选连接器 ID
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerMessageRequest {
    /// 要触发的消息类型（例如 Heartbeat、MeterValues）
    pub requested_message: MessageTrigger,
    /// 可选的连接器编号（某些消息需指定连接器）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

/// TriggerMessage 处理器接口
pub trait TriggerMessageHandler: Send + Sync {
    fn handle(&self, req: TriggerMessageRequest) -> TriggerMessageConfirmation;
}

/// 默认实现：接受触发请求
pub struct DefaultTriggerMessageHandler;

impl Default for DefaultTriggerMessageHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultTriggerMessageHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl TriggerMessageHandler for DefaultTriggerMessageHandler {
    fn handle(&self, _req: TriggerMessageRequest) -> TriggerMessageConfirmation {
        TriggerMessageConfirmation::accepted()
    }
}
