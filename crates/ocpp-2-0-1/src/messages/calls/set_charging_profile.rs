//! SetChargingProfile Request (Functional Block A)
//! 设置充电曲线

use serde::{Deserialize, Serialize};
use crate::common::ChargingProfileType;

/// SetChargingProfile 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    /// EVSE ID
    pub evse_id: i32,
    /// 充电曲线
    pub charging_profile: ChargingProfileType,
}

impl SetChargingProfileRequest {
    pub fn new(evse_id: i32, charging_profile: ChargingProfileType) -> Self {
        Self {
            evse_id,
            charging_profile,
        }
    }
}

pub const ACTION: &str = "SetChargingProfile";