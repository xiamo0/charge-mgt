//! OCPP 2.0.1 / 2.1 Firmware types (Functional Block L)

use serde::{Deserialize, Serialize};

/// 固件类型 (UpdateFirmware.req)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    pub location: String,
    pub retrieve_date_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl FirmwareType {
    pub fn new(location: impl Into<String>, retrieve_date_time: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            retrieve_date_time: retrieve_date_time.into(),
            install_date_time: None,
            signing_certificate: None,
            signature: None,
        }
    }
}

/// 固件更新状态枚举 (FirmwareStatusNotification)
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

/// 固件发布状态枚举 (PublishFirmwareStatusNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PublishFirmwareStatusEnumType {
    DownloadScheduled,
    DownloadPaused,
    Downloading,
    Downloaded,
    DownloadFailed,
    DownloadVerificationFailed,
    DownloadVerificationVerified,
    PublishFailed,
    Published,
}
