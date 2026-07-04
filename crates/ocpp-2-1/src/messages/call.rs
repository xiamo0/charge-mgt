//! OCPP 2.1 Call Factory Functions

use crate::common::uuid::generate_uuid;
use crate::messages::envelope::{Call, CallResult, Send};
use serde::Serialize;

/// 创建一个新的 Call 消息
pub fn create_new_call<T: Serialize>(action: &str, payload: &T) -> Call {
    Call::new(
        generate_uuid(),
        action,
        serde_json::to_value(payload).unwrap_or_default(),
    )
}

/// 创建一个新的 CallResult 消息
pub fn create_new_call_result<T: Serialize>(unique_id: &str, payload: &T) -> CallResult {
    CallResult::new(
        unique_id,
        serde_json::to_value(payload).unwrap_or_default(),
    )
}

/// 创建一个新的 Send 消息 (2.1 新增，单向 fire-and-forget)
pub fn create_send<T: Serialize>(action: &str, payload: &T) -> Send {
    Send::new(
        generate_uuid(),
        action,
        serde_json::to_value(payload).unwrap_or_default(),
    )
}
