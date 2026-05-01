//! ClearChargingProfile 消息及处理器

use super::super::confs::clear_charging_profile_conf::ClearChargingProfileConfirmation;
use crate::common::status::ChargingProfilePurpose;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearChargingProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

pub trait ClearChargingProfileHandler: Send + Sync {
    fn handle(&self, req: ClearChargingProfileRequest) -> ClearChargingProfileConfirmation;
}

pub struct DefaultClearChargingProfileHandler;

impl Default for DefaultClearChargingProfileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultClearChargingProfileHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ClearChargingProfileHandler for DefaultClearChargingProfileHandler {
    fn handle(&self, _req: ClearChargingProfileRequest) -> ClearChargingProfileConfirmation {
        ClearChargingProfileConfirmation::accepted()
    }
}
