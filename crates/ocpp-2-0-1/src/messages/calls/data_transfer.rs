//! DataTransfer Request (Functional Block P)
//! 厂商自定义数据传输

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 数据传输状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DataTransferStatusEnumType {
    Accepted,
    Rejected,
    UnknownVendorId,
    UnknownMessageId,
}

/// DataTransfer 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// 厂商 ID (max 255 chars)
    pub vendor_id: String,
    /// 消息 ID (可选, max 50 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 自定义数据 (任意 JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl DataTransferRequest {
    pub fn new(vendor_id: impl Into<String>) -> Self {
        Self {
            vendor_id: vendor_id.into(),
            message_id: None,
            data: None,
        }
    }

    /// 设置消息 ID
    pub fn with_message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    /// 设置自定义数据
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }
}

pub const ACTION: &str = "DataTransfer";