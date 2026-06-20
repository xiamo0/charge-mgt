//! BootNotification 消息及处理器
//!
//! 定义 BootNotification 请求结构、处理器 trait 以及默认实现。BootNotification 用于设备启动时向集线器登记自身信息。

use super::super::confs::boot_notification_conf::BootNotificationConfirmation;
use serde::{Deserialize, Serialize};

/// BootNotification 请求，包含充电点厂商与型号等可选信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BootNotificationRequest {
    /// 充电点厂商
    #[serde(rename = "chargePointVendor")]
    pub charge_point_vendor: String,
    /// 充电点型号
    #[serde(rename = "chargePointModel")]
    pub charge_point_model: String,
    /// 充电盒序列号（可选）
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "chargeBoxSerialNumber"
    )]
    pub charge_box_serial_number: Option<String>,
    /// 充电点序列号（可选）
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "chargePointSerialNumber"
    )]
    pub charge_point_serial_number: Option<String>,
    /// 固件版本（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    /// ICCID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    /// IMSI（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    /// 计量表类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
    /// 计量表序列号（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
}

/// BootNotification 的处理器接口
pub trait BootNotificationHandler: Send + Sync {
    fn handle(&self, req: BootNotificationRequest) -> BootNotificationConfirmation;
}

/// BootNotification 的本地配置（响应中的返回状态与间隔）
#[derive(Debug, Clone)]
pub struct BootNotificationConfig {
    /// 启动登记结果状态（Accepted / Pending / Rejected）
    pub status: crate::common::status::RegistrationStatus,
    /// 如果 Accepted，服务器建议的间隔（秒）
    pub interval_secs: i32,
}

impl Default for BootNotificationConfig {
    fn default() -> Self {
        Self {
            status: crate::common::status::RegistrationStatus::Accepted,
            interval_secs: 30,
        }
    }
}

/// 默认的 BootNotification 处理器，返回配置中的默认响应
pub struct DefaultBootNotificationHandler {
    config: BootNotificationConfig,
}

impl DefaultBootNotificationHandler {
    /// 使用给定配置创建处理器
    pub fn new(config: BootNotificationConfig) -> Self {
        Self { config }
    }
}

impl BootNotificationHandler for DefaultBootNotificationHandler {
    fn handle(&self, _req: BootNotificationRequest) -> BootNotificationConfirmation {
        let now = chrono::Utc::now().to_rfc3339();
        BootNotificationConfirmation::accepted(&now, self.config.interval_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_boot_notification_request_roundtrip() {
        let req = BootNotificationRequest {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "ModelY".to_string(),
            charge_box_serial_number: Some("SN123".to_string()),
            charge_point_serial_number: None,
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        let de: BootNotificationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req.charge_point_vendor, de.charge_point_vendor);
        assert_eq!(req.charge_box_serial_number, de.charge_box_serial_number);
    }

    #[test]
    fn test_boot_notification_config_default() {
        let config = BootNotificationConfig::default();
        assert_eq!(
            config.status,
            crate::common::status::RegistrationStatus::Accepted
        );
        assert_eq!(config.interval_secs, 30);
    }

    #[test]
    fn test_default_boot_notification_handler_new() {
        let config = BootNotificationConfig::default();
        let handler = DefaultBootNotificationHandler::new(config);
        let req = BootNotificationRequest {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "ModelY".to_string(),
            charge_box_serial_number: None,
            charge_point_serial_number: None,
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };
        let conf = handler.handle(req);
        assert_eq!(
            conf.status,
            crate::common::status::RegistrationStatus::Accepted
        );
        assert_eq!(conf.interval, 30);
    }

    #[test]
    fn test_default_boot_notification_handler_thread_safe() {
        let config = BootNotificationConfig::default();
        let handler: Arc<dyn BootNotificationHandler> =
            Arc::new(DefaultBootNotificationHandler::new(config));
        let req = BootNotificationRequest {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "ModelY".to_string(),
            charge_box_serial_number: None,
            charge_point_serial_number: None,
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };
        let conf = handler.handle(req);
        assert_eq!(
            conf.status,
            crate::common::status::RegistrationStatus::Accepted
        );
    }
}
