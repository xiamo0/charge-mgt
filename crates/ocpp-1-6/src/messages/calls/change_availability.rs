//! ChangeAvailability 消息及处理器

use super::super::confs::change_availability_conf::ChangeAvailabilityConfirmation;
use crate::common::status::AvailabilityType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeAvailabilityRequest {
    pub connector_id: i32,
    #[serde(rename = "type")]
    pub availability_type: AvailabilityType,
}

pub trait ChangeAvailabilityHandler: Send + Sync {
    fn handle(&self, req: ChangeAvailabilityRequest) -> ChangeAvailabilityConfirmation;
}

pub struct DefaultChangeAvailabilityHandler;

impl Default for DefaultChangeAvailabilityHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultChangeAvailabilityHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ChangeAvailabilityHandler for DefaultChangeAvailabilityHandler {
    fn handle(&self, _req: ChangeAvailabilityRequest) -> ChangeAvailabilityConfirmation {
        ChangeAvailabilityConfirmation::accepted()
    }
}
