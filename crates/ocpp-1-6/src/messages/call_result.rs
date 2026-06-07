//! CallResult 消息类型
//!
//! 定义 OCPP CallResult 消息相关的类型别名和构造函数。
//! OCPP CallResult 消息格式为数组: [3, "<uniqueId>", {payload}]

pub use super::envelope::CallResult;
use serde_json::Value;

/// CallResult 的载荷类型（任意 JSON 值）
pub type ResultPayload = Value;

/// 快速构造一个 `CallResult` 实例的 helper
///
/// 传入对应的 unique_id（与 Call 的 uniqueId 对应）和 JSON 载荷，返回封装好的 CallResult。
pub fn new_call_result(unique_id: &str, payload: Value) -> CallResult {
    CallResult::new(unique_id, payload)
}
