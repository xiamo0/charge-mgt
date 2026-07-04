//! UnpublishFirmware Request (Functional Block I)
//! 停止固件发布

use serde::{Deserialize, Serialize};

/// UnpublishFirmware 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareRequest {
    /// 校验和 (之前发布的固件的 checksum)
    pub checksum: String,
}

impl UnpublishFirmwareRequest {
    pub fn new(checksum: impl Into<String>) -> Self {
        Self {
            checksum: checksum.into(),
        }
    }
}

pub const ACTION: &str = "UnpublishFirmware";
