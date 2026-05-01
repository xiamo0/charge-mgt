//! Call 消息类型

pub use super::envelope::Call;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageId(pub String);

impl MessageId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

pub type Payload = serde_json::Value;

pub fn new_call(action: &str, payload: Payload) -> Call {
    Call::new(action, &MessageId::new().0, payload)
}