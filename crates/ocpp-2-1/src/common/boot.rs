//! Boot Notification Types (Functional Block B)

use serde::{Deserialize, Serialize};

/// 重启原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BootReasonEnumType {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}

/// 调制解调器类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}

impl Default for ModemType {
    fn default() -> Self { Self { iccid: None, imsi: None } }
}

impl ModemType {
    pub fn new() -> Self { Self::default() }
}

/// 充电站类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    pub model: String,
    pub vendor_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}

impl ChargingStationType {
    pub fn new(model: impl Into<String>, vendor_name: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            vendor_name: vendor_name.into(),
            serial_number: None,
            firmware_version: None,
            modem: None,
        }
    }
}

/// 状态信息类型 (广泛复用于各响应)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    pub reason_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl StatusInfoType {
    pub fn new(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
            additional_info: None,
        }
    }
}
