//! PublishFirmware Request (Functional Block I)
//! 发布固件供本地分发

use serde::{Deserialize, Serialize};

/// PublishFirmware 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    /// 固件位置 URL
    pub location: String,
    /// MD5 校验和
    pub checksum: String,
    /// 请求 ID
    pub request_id: i32,
    /// 重试次数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// 重试间隔 (秒, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

impl PublishFirmwareRequest {
    pub fn new(location: impl Into<String>, checksum: impl Into<String>, request_id: i32) -> Self {
        Self {
            location: location.into(),
            checksum: checksum.into(),
            request_id,
            retries: None,
            retry_interval: None,
        }
    }
}

pub const ACTION: &str = "PublishFirmware";