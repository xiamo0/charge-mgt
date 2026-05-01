//! BootNotification 消息及处理器

use super::super::confs::boot_notification_conf::BootNotificationConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BootNotificationRequest {
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_serial_number: Option<String>,
}

pub trait BootNotificationHandler: Send + Sync {
    fn handle(&self, req: BootNotificationRequest) -> BootNotificationConfirmation;
}

#[derive(Debug, Clone)]
pub struct BootNotificationConfig {
    pub status: crate::common::status::RegistrationStatus,
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

pub struct DefaultBootNotificationHandler {
    config: BootNotificationConfig,
}

impl DefaultBootNotificationHandler {
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
