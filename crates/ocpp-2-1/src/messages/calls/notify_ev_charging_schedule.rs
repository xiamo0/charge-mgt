//! NotifyEVChargingSchedule Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::ChargingScheduleType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    pub time_base: String,
    pub charging_schedule: ChargingScheduleType,
    pub evse_id: i32,
}

pub const ACTION: &str = "NotifyEVChargingSchedule";
