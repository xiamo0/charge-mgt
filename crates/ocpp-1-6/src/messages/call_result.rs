//! CallResult 消息类型

pub use super::envelope::CallResult;
use serde_json::Value;

pub type ResultPayload = Value;

pub fn new_call_result(unique_id: &str, payload: Value) -> CallResult {
    CallResult::new(unique_id, payload)
}