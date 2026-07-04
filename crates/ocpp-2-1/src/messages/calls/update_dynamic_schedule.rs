//! UpdateDynamicSchedule Request (Block K — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::ChargingScheduleUpdateType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDynamicScheduleRequest {
    pub charging_profile_id: i32,
    pub schedule_update: ChargingScheduleUpdateType,
}

pub const ACTION: &str = "UpdateDynamicSchedule";
