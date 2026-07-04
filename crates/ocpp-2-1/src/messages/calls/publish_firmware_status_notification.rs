//! PublishFirmwareStatusNotification Request (Block L)
use serde::{Deserialize, Serialize};
use crate::common::PublishFirmwareStatusEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    pub status: PublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
}

pub const ACTION: &str = "PublishFirmwareStatusNotification";
