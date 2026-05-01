//! StatusNotification 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::status_notification_conf::StatusNotificationConfirmation;
use crate::common::status::{ChargePointStatus, ChargePointErrorCode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusNotificationRequest {
    pub connector_id: i32,
    pub error_code: ChargePointErrorCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    pub status: ChargePointStatus,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_error_code: Option<String>,
}

pub trait StatusNotificationHandler: Send + Sync {
    fn handle(&self, req: StatusNotificationRequest) -> StatusNotificationConfirmation;
}

pub struct DefaultStatusNotificationHandler;

impl Default for DefaultStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStatusNotificationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl StatusNotificationHandler for DefaultStatusNotificationHandler {
    fn handle(&self, _req: StatusNotificationRequest) -> StatusNotificationConfirmation {
        StatusNotificationConfirmation
    }
}