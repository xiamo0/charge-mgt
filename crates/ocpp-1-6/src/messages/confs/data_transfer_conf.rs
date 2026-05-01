//! DataTransfer 响应

use serde::{Deserialize, Serialize};
use crate::common::status::DataTransferStatus;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataTransferConfirmation {
    pub status: DataTransferStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl DataTransferConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: DataTransferStatus::Accepted,
            data: None,
        }
    }

    pub fn unknown_vendor_id() -> Self {
        Self {
            status: DataTransferStatus::UnknownVendorId,
            data: None,
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: DataTransferStatus::Rejected,
            data: None,
        }
    }
}