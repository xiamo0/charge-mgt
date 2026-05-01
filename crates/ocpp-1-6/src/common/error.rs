//! OCPP 错误类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum ErrorCode {
    NotImplemented,
    NotSupported,
    InternalError,
    ProtocolError,
    SecurityError,
    FormationViolation,
    PropertyConstraintViolation,
    OccurenceConstraintViolation,
    TypeConstraintViolation,
    #[default]
    GenericError,
}

#[derive(Debug, thiserror::Error)]
pub enum OcppError {
    #[error("协议错误: {0}")]
    Protocol(String),
    #[error("验证错误: {0}")]
    Validation(String),
    #[error("未实现")]
    NotImplemented,
    #[error("不支持")]
    NotSupported,
    #[error("内部错误: {0}")]
    Internal(String),
    #[error("通用错误: {0}")]
    Generic(String),
}