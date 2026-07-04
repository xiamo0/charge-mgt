//! SetChargingProfile Request (Block K)
use crate::common::ChargingProfileType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub evse_id: i32,
    pub charging_profile: ChargingProfileType,
}

pub const ACTION: &str = "SetChargingProfile";
