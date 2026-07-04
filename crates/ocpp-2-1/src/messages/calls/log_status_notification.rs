//! LogStatusNotification Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::UploadLogStatusEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    pub status: UploadLogStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

pub const ACTION: &str = "LogStatusNotification";
