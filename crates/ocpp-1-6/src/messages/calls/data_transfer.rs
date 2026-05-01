//! DataTransfer 消息及处理器

use super::super::confs::data_transfer_conf::DataTransferConfirmation;
use crate::common::status::DataTransferStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, thiserror::Error)]
pub enum DataTransferError {
    #[error("未知厂商")]
    UnknownVendor,
    #[error("拒绝")]
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTransferRequest {
    pub vendor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

pub trait DataTransferHandler: Send + Sync {
    fn vendor_id(&self) -> &'static str;
    fn handle(
        &self,
        message_id: Option<&str>,
        data: &Option<JsonValue>,
    ) -> Result<JsonValue, DataTransferError>;
}

pub struct DefaultDataTransferHandler;

impl Default for DefaultDataTransferHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultDataTransferHandler {
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
