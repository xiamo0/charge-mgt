//! FirmwareStatusNotification Request (Block L)
use serde::{Deserialize, Serialize};
use crate::common::FirmwareStatusEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

pub const ACTION: &str = "FirmwareStatusNotification";
