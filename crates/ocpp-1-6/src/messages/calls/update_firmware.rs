//! UpdateFirmware 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::update_firmware_conf::UpdateFirmwareConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFirmwareRequest {
    pub location: String,
    pub retries: Option<i32>,
    pub retry_interval: Option<i32>,
    pub request_id: i32,
    pub retrieve_time: String,
}

pub trait UpdateFirmwareHandler: Send + Sync {
    fn handle(&self, req: UpdateFirmwareRequest) -> UpdateFirmwareConfirmation;
}

pub struct DefaultUpdateFirmwareHandler;

impl Default for DefaultUpdateFirmwareHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultUpdateFirmwareHandler {
    pub fn new() -> Self {
        Self
    }
}

impl UpdateFirmwareHandler for DefaultUpdateFirmwareHandler {
    fn handle(&self, _req: UpdateFirmwareRequest) -> UpdateFirmwareConfirmation {
        UpdateFirmwareConfirmation
    }
}