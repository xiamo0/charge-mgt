//! Call 消息类型
//!
//! 该模块定义用于构建和序列化 OCPP "Call" 消息的辅助类型和函数。
//! OCPP Call 消息格式为数组: [2, "<uniqueId>", "<action>", {payload}]

pub use super::envelope::Call;
use serde::{Deserialize, Serialize};

/// 消息 ID 类型封装（UUID 字符串）
///
/// 使用新生成的 UUID 作为默认值，便于创建唯一的 Call 消息标识。
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageId(pub String);

impl MessageId {
    /// 生成一个新的随机 UUID 字符串作为消息 ID
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

/// Call 消息的载荷类型（任意 JSON 值）
pub type Payload = serde_json::Value;

/// 快速创建一个 `Call` 实例的 helper
///
/// 传入 action 名称和一个 JSON 载荷，函数会生成一个新的 message id 并构造 `Call`。
pub fn new_call(action: &str, payload: Payload) -> Call {
    Call::new(action, &MessageId::new().0, payload)
}
