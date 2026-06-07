//! DataTransfer 消息及处理器
//!
//! DataTransfer 用于厂商自定义的透传数据交换。此处定义请求结构、处理器 trait 与默认实现，以及把处理器结果转换为确认的辅助方法。

use super::super::confs::data_transfer_conf::DataTransferConfirmation;
use crate::common::status::DataTransferStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// DataTransfer 处理可能返回的错误类型
#[derive(Debug, thiserror::Error)]
pub enum DataTransferError {
    #[error("未知厂商")]
    UnknownVendor,
    #[error("拒绝")]
    Rejected,
}

/// DataTransfer 请求，包含厂商 ID、可选的 message_id 与自定义数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTransferRequest {
    /// 厂商标识
    pub vendor_id: String,
    /// 可选的消息 ID（由上层传递，用于追踪）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 厂商自定义的任意 JSON 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

/// DataTransfer 处理器接口：提供厂商 ID 和处理函数，返回处理结果或错误
pub trait DataTransferHandler: Send + Sync {
    fn vendor_id(&self) -> &'static str;
    fn handle(
        &self,
        message_id: Option<&str>,
        data: &Option<JsonValue>,
    ) -> Result<JsonValue, DataTransferError>;
}

/// 默认空实现的 DataTransfer 处理器（默认返回 UnknownVendor）
pub struct DefaultDataTransferHandler;

impl Default for DefaultDataTransferHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultDataTransferHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl DataTransferHandler for DefaultDataTransferHandler {
    fn vendor_id(&self) -> &'static str {
        "default"
    }

    fn handle(
        &self,
        _message_id: Option<&str>,
        _data: &Option<JsonValue>,
    ) -> Result<JsonValue, DataTransferError> {
        Err(DataTransferError::UnknownVendor)
    }
}

impl DataTransferConfirmation {
    /// 将处理器返回的 Result 转换为 DataTransferConfirmation（封装状态与可选数据）
    pub fn from_handler_result(result: Result<JsonValue, DataTransferError>) -> Self {
        match result {
            Ok(data) => Self {
                status: DataTransferStatus::Accepted,
                data: Some(data),
            },
            Err(DataTransferError::UnknownVendor) => Self::unknown_vendor_id(),
            Err(DataTransferError::Rejected) => Self::rejected(),
        }
    }
}
