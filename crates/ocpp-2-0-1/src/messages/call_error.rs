//! OCPP 2.0.1 CallError Factory Functions

use crate::messages::envelope::{CallError, error_codes};

/// 创建一个新的 CallError 消息
pub fn create_call_error(unique_id: &str, error_code: &str, error_description: &str) -> CallError {
    CallError::new(unique_id, error_code, error_description)
}

/// Not Implemented
pub fn not_implemented(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::NOT_IMPLEMENTED, description)
}

/// Not Supported
pub fn not_supported(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::NOT_SUPPORTED, description)
}

/// Internal Error
pub fn internal_error(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::INTERNAL_ERROR, description)
}

/// Protocol Error
pub fn protocol_error(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::PROTOCOL_ERROR, description)
}

/// Security Error
pub fn security_error(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::SECURITY_ERROR, description)
}

/// Format Violation Error
pub fn format_violation_error(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::FORMAT_VIOLATION_ERROR, description)
}

/// Property Constraint Violation
pub fn property_constraint_violation(unique_id: &str, description: &str) -> CallError {
    CallError::new(
        unique_id,
        error_codes::PROPERTY_CONSTRAINT_VIOLATION,
        description,
    )
}

/// Occurrence Constraint Violation
pub fn occurrence_constraint_violation(unique_id: &str, description: &str) -> CallError {
    CallError::new(
        unique_id,
        error_codes::OCCURRENCE_CONSTRAINT_VIOLATION,
        description,
    )
}

/// Type Constraint Violation
pub fn type_constraint_violation(unique_id: &str, description: &str) -> CallError {
    CallError::new(
        unique_id,
        error_codes::TYPE_CONSTRAINT_VIOLATION,
        description,
    )
}

/// Generic Error
pub fn generic_error(unique_id: &str, description: &str) -> CallError {
    CallError::new(unique_id, error_codes::GENERIC_ERROR, description)
}
