//! PublishFirmwareStatusNotification Request (Functional Block I)
//! 固件发布状态通知

use serde::{Deserialize, Serialize};

/// 发布固件状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PublishFirmwareStatusEnumType {
    Idle,
    DownloadScheduled,
    Downloading,
    Downloaded,
    DownloadFailed,
    DownloadPaused,
    InvalidChecksum,
    CheckSumVerified,
    PublishFailed,
}

/// PublishFirmwareStatusNotification 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    /// 发布状态
    pub status: PublishFirmwareStatusEnumType,
    /// 位置列表 (可选, 下载完成后列出位置)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    /// 请求 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

impl PublishFirmwareStatusNotificationRequest {
    pub fn new(status: PublishFirmwareStatusEnumType) -> Self {
        Self {
            status,
            location: None,
            request_id: None,
        }
    }

    pub fn with_location(mut self, location: Vec<String>) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_request_id(mut self, request_id: i32) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

pub const ACTION: &str = "PublishFirmwareStatusNotification";