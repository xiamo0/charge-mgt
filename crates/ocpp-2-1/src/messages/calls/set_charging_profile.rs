//! SetChargingProfile Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::ChargingProfileType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub evse_id: i32,
    pub charging_profile: ChargingProfileType,
}

pub const ACTION: &str = "SetChargingProfile";
