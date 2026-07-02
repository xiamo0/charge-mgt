//! FirmwareStatusNotification Request (Functional Block I)
//! 固件更新状态通知

use serde::{Deserialize, Serialize};

/// 固件状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FirmwareStatusEnumType {
    Downloaded,
    DownloadFailed,
    Downloading,
    DownloadScheduled,
    DownloadPaused,
    Idle,
    InstallationFailed,
    Installing,
    Installed,
    InstallRebooting,
    InstallScheduled,
    InstallVerificationFailed,
    InvalidSignature,
    SignatureVerified,
}

/// FirmwareStatusNotification 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    /// 固件状态
    pub status: FirmwareStatusEnumType,
    /// 请求 ID (可选, 关联 UpdateFirmware 的 requestId)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

impl FirmwareStatusNotificationRequest {
    pub fn new(status: FirmwareStatusEnumType) -> Self {
        Self {
            status,
            request_id: None,
        }
    }

    pub fn with_request_id(mut self, request_id: i32) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

pub const ACTION: &str = "FirmwareStatusNotification";