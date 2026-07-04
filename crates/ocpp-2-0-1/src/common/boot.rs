//! Boot Notification Types (Functional Block B)

use serde::{Deserialize, Serialize};

/// 重启原因枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BootReasonEnumType {
    /// 应用复位
    ApplicationReset,
    /// 固件更新后
    FirmwareUpdate,
    /// 本地复位
    LocalReset,
    /// 上电启动
    PowerUp,
    /// 远程复位
    RemoteReset,
    /// 计划复位
    ScheduledReset,
    /// 被触发
    Triggered,
    /// 未知
    Unknown,
    /// 看门狗复位
    Watchdog,
}

/// 调制解调器类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// ICCID (max 20 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    /// IMSI (max 20 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}

impl ModemType {
    /// 创建新的调制解调器信息
    pub fn new() -> Self {
        Self {
            iccid: None,
            imsi: None,
        }
    }

    /// 设置 ICCID
    pub fn with_iccid(mut self, iccid: impl Into<String>) -> Self {
        self.iccid = Some(iccid.into());
        self
    }

    /// 设置 IMSI
    pub fn with_imsi(mut self, imsi: impl Into<String>) -> Self {
        self.imsi = Some(imsi.into());
        self
    }
}

impl Default for ModemType {
    fn default() -> Self {
        Self::new()
    }
}

/// 充电站类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// 型号 (max 20 chars)
    pub model: String,
    /// 厂商名称 (max 50 chars)
    pub vendor_name: String,
    /// 序列号 (max 25 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// 固件版本 (max 50 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// 调制解调器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,
}

impl ChargingStationType {
    /// 创建新的充电站信息
    pub fn new(model: impl Into<String>, vendor_name: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            vendor_name: vendor_name.into(),
            serial_number: None,
            firmware_version: None,
            modem: None,
        }
    }

    /// 设置序列号
    pub fn with_serial_number(mut self, serial_number: impl Into<String>) -> Self {
        self.serial_number = Some(serial_number.into());
        self
    }

    /// 设置固件版本
    pub fn with_firmware_version(mut self, firmware_version: impl Into<String>) -> Self {
        self.firmware_version = Some(firmware_version.into());
        self
    }

    /// 设置调制解调器
    pub fn with_modem(mut self, modem: ModemType) -> Self {
        self.modem = Some(modem);
        self
    }
}

/// 充电站状态枚举 (StatusNotification)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StatusInfoEnumType {
    /// 状态信息
    StatusInfo,
}

/// 状态信息类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    /// 原因代码
    pub reason_code: String,
    /// 附加信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl StatusInfoType {
    /// 创建新的状态信息
    pub fn new(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
            additional_info: None,
        }
    }

    /// 添加附加信息
    pub fn with_additional_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_reason_enum() {
        let variants = [
            BootReasonEnumType::ApplicationReset,
            BootReasonEnumType::FirmwareUpdate,
            BootReasonEnumType::LocalReset,
            BootReasonEnumType::PowerUp,
            BootReasonEnumType::RemoteReset,
            BootReasonEnumType::ScheduledReset,
            BootReasonEnumType::Triggered,
            BootReasonEnumType::Unknown,
            BootReasonEnumType::Watchdog,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: BootReasonEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_modem_type() {
        let modem = ModemType::new()
            .with_iccid("89012345678901234567")
            .with_imsi("460001234567890");
        let json = serde_json::to_string(&modem).unwrap();
        let de: ModemType = serde_json::from_str(&json).unwrap();
        assert_eq!(modem, de);
        assert_eq!(modem.iccid, Some("89012345678901234567".to_string()));
    }

    #[test]
    fn test_charging_station_type() {
        let station = ChargingStationType::new("ModelX", "VendorA")
            .with_serial_number("SN12345")
            .with_firmware_version("1.2.3")
            .with_modem(ModemType::new().with_iccid("89012345678901234567"));
        let json = serde_json::to_string(&station).unwrap();
        let de: ChargingStationType = serde_json::from_str(&json).unwrap();
        assert_eq!(station, de);
    }

    #[test]
    fn test_status_info_type() {
        let info = StatusInfoType::new("ErrorCode").with_additional_info("Some additional info");
        let json = serde_json::to_string(&info).unwrap();
        let de: StatusInfoType = serde_json::from_str(&json).unwrap();
        assert_eq!(info, de);
    }
}
