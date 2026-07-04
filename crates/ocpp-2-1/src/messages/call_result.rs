//! OCPP 2.1 CallResult Factory Functions

use crate::messages::envelope::CallResult;
use serde::Serialize;

/// 创建一个新的 CallResult 消息
pub fn create_new_call_result<T: Serialize>(unique_id: &str, payload: &T) -> CallResult {
    CallResult::new(
        unique_id,
        serde_json::to_value(payload).unwrap_or_default(),
    )
}
