//! UpdateFirmware Request (Block L)
use crate::common::FirmwareType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    pub request_id: i32,
    pub firmware: FirmwareType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

pub const ACTION: &str = "UpdateFirmware";
