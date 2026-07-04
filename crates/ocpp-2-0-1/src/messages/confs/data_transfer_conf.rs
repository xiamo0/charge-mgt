//! DataTransfer Confirmation
use crate::messages::calls::data_transfer::DataTransferStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferConfirmation {
    pub status: DataTransferStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
