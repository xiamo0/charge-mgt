//! CallError 消息类型

pub use super::envelope::CallError;

pub fn new_call_error(unique_id: &str, error_code: &str, error_description: &str) -> CallError {
    CallError::new(unique_id, error_code, error_description)
}

pub mod errors {
    use super::*;

    pub fn not_implemented(unique_id: &str) -> CallError {
        new_call_error(unique_id, "NotImplemented", "操作未实现")
    }

    pub fn not_supported(unique_id: &str) -> CallError {
        new_call_error(unique_id, "NotSupported", "操作不支持")
    }

    pub fn internal_error(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "InternalError", msg)
    }

    pub fn protocol_error(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "ProtocolError", msg)
    }

    pub fn formation_violation(unique_id: &str, msg: &str) -> CallError {
        new_call_error(unique_id, "FormationViolation", msg)
    }
}