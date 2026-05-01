//! SetChargingProfile 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::set_charging_profile_conf::SetChargingProfileConfirmation;
use crate::common::configuration::ChargingProfile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetChargingProfileRequest {
    pub connector_id: i32,
    pub charging_profile: ChargingProfile,
}

pub trait SetChargingProfileHandler: Send + Sync {
    fn handle(&self, req: SetChargingProfileRequest) -> SetChargingProfileConfirmation;
}

pub struct DefaultSetChargingProfileHandler;

impl Default for DefaultSetChargingProfileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSetChargingProfileHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SetChargingProfileHandler for DefaultSetChargingProfileHandler {
    fn handle(&self, _req: SetChargingProfileRequest) -> SetChargingProfileConfirmation {
        SetChargingProfileConfirmation::accepted()
    }
}