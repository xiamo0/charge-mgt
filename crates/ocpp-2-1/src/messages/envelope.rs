//! OCPP 2.1 Message Envelope Types
//! 消息封装类型，定义 Call/CallResult/CallError/CallResultError/Send 的序列化和反序列化

use serde::{Deserialize, Serialize};

/// OCPP 2.1 Call 消息 (请求)
/// 格式: [2, "messageId", "action", payload]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub message_type_id: i32,
    pub unique_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

impl Call {
    pub const MESSAGE_TYPE_ID: i32 = 2;

    pub fn new(
        unique_id: impl Into<String>,
        action: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            message_type_id: Self::MESSAGE_TYPE_ID,
            unique_id: unique_id.into(),
            action: action.into(),
            payload,
        }
    }
}

/// OCPP 2.1 CallResult 消息 (成功响应)
/// 格式: [3, "messageId", payload]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResult {
    pub message_type_id: i32,
    pub unique_id: String,
    pub payload: serde_json::Value,
}

impl CallResult {
    pub const MESSAGE_TYPE_ID: i32 = 3;

    pub fn new(unique_id: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            message_type_id: Self::MESSAGE_TYPE_ID,
            unique_id: unique_id.into(),
            payload,
        }
    }
}

/// OCPP 2.1 CallError 消息 (错误响应)
/// 格式: [4, "messageId", "errorCode", "errorDescription", "errorDetails"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallError {
    pub message_type_id: i32,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: serde_json::Value,
}

impl CallError {
    pub const MESSAGE_TYPE_ID: i32 = 4;

    pub fn new(
        unique_id: impl Into<String>,
        error_code: impl Into<String>,
        error_description: impl Into<String>,
    ) -> Self {
        Self {
            message_type_id: Self::MESSAGE_TYPE_ID,
            unique_id: unique_id.into(),
            error_code: error_code.into(),
            error_description: error_description.into(),
            error_details: serde_json::json!({}),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.error_details = details;
        self
    }
}

/// OCPP 2.1 CallResultError 消息 (2.1 新增，对 CallResult payload 处理失败时返回)
/// 格式: [5, "messageId", "errorCode", "errorDescription", "errorDetails"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResultError {
    pub message_type_id: i32,
    pub unique_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: serde_json::Value,
}

impl CallResultError {
    pub const MESSAGE_TYPE_ID: i32 = 5;

    pub fn new(
        unique_id: impl Into<String>,
        error_code: impl Into<String>,
        error_description: impl Into<String>,
    ) -> Self {
        Self {
            message_type_id: Self::MESSAGE_TYPE_ID,
            unique_id: unique_id.into(),
            error_code: error_code.into(),
            error_description: error_description.into(),
            error_details: serde_json::json!({}),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.error_details = details;
        self
    }
}

/// OCPP 2.1 Send 消息 (2.1 新增，单向 fire-and-forget，无响应)
/// 格式: [6, "messageId", "action", payload]
/// 用于高频遥测（NotifyPeriodicEventStream）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Send {
    pub message_type_id: i32,
    pub unique_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

impl Send {
    pub const MESSAGE_TYPE_ID: i32 = 6;

    pub fn new(
        unique_id: impl Into<String>,
        action: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            message_type_id: Self::MESSAGE_TYPE_ID,
            unique_id: unique_id.into(),
            action: action.into(),
            payload,
        }
    }
}

/// OCPP 2.0.1/2.1 错误代码枚举 (使用 FormatViolationError 而非 1.6 的 FormationViolation)
pub mod error_codes {
    pub const NOT_IMPLEMENTED: &str = "NotImplemented";
    pub const NOT_SUPPORTED: &str = "NotSupported";
    pub const INTERNAL_ERROR: &str = "InternalError";
    pub const PROTOCOL_ERROR: &str = "ProtocolError";
    pub const SECURITY_ERROR: &str = "SecurityError";
    pub const FORMAT_VIOLATION_ERROR: &str = "FormatViolationError";
    pub const PROPERTY_CONSTRAINT_VIOLATION: &str = "PropertyConstraintViolation";
    pub const OCCURRENCE_CONSTRAINT_VIOLATION: &str = "OccurrenceConstraintViolation";
    pub const TYPE_CONSTRAINT_VIOLATION: &str = "TypeConstraintViolation";
    pub const GENERIC_ERROR: &str = "GenericError";
}
