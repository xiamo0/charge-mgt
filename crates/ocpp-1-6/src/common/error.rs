//! OCPP 错误类型
//!
//! 定义协议级别的错误码枚举以及运行时错误类型 `OcppError`，用于在库内部和上层逻辑中表示不同的错误情形。

use serde::{Deserialize, Serialize};

/// OCPP 标准错误码枚举（用于 CallError.errorCode 字段）
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
    /// 默认通用错误
    #[default]
    GenericError,
}

/// 库内部使用的错误类型，基于 `thiserror` 以便简洁地定义 Display/Source
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
