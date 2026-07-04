//! UnpublishFirmware Request (Block L)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    pub checksum: String,
}

pub const ACTION: &str = "UnpublishFirmware";
