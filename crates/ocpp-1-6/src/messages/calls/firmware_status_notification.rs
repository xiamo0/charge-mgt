//! FirmwareStatusNotification 消息及处理器（单向消息，无需回复）

use serde::{Deserialize, Serialize};
use crate::common::status::FirmwareStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirmwareStatusNotificationRequest {
    pub status: FirmwareStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

pub trait FirmwareStatusNotificationHandler: Send + Sync {
    fn handle(&self, req: FirmwareStatusNotificationRequest);
}

pub struct DefaultFirmwareStatusNotificationHandler;

impl Default for DefaultFirmwareStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultFirmwareStatusNotificationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl FirmwareStatusNotificationHandler for DefaultFirmwareStatusNotificationHandler {
    fn handle(&self, _req: FirmwareStatusNotificationRequest) {
    }
}