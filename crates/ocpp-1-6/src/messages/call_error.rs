//! CallError 消息类型
//!
//! 定义 OCPP CallError 消息的构造 helper 以及常用错误构造函数。
//! OCPP CallError 消息格式为数组: [4, "<uniqueId>", "<errorCode>", "<errorDescription>", {errorDetails}]

pub use super::envelope::CallError;

/// 快速构造一个 `CallError` 实例（不包含 errorDetails）
pub fn new_call_error(unique_id: &str, error_code: &str, error_description: &str) -> CallError {
    CallError::new(unique_id, error_code, error_description)
}

/// 常用错误构造器集合，便于快速返回标准的 OCPP 错误响应
pub mod errors {
    use super::*;

    /// 未实现（NotImplemented）错误
    pub fn not_implemented(unique_id: &str) -> CallError {
        new_call_error(unique_id, "NotImplemented", "操作未实现")
    }

    /// 不支持（NotSupported）错误
    pub fn not_supported(unique_id: &str) -> CallError {
        new_call_error(unique_id, "NotSupported", "操作不支持")
    }

    /// 内部错误（InternalError），可携带自定义描述
    pub fn internal_error(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "InternalError", msg)
    }

    /// 协议错误（ProtocolError），可携带自定义描述
    pub fn protocol_error(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "ProtocolError", msg)
    }

    /// 格式/形成（FormationViolation）错误，用于消息格式不符合规范的情况
    pub fn formation_violation(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "FormationViolation", msg)
    }
}
