//! NotifyEVChargingSchedule Request (Functional Block B - ISO 15118)
//! 转发 EV 充电计划

use serde::{Deserialize, Serialize};
use crate::common::ChargingScheduleType;

/// NotifyEVChargingSchedule 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    /// 时间点
    pub time_base: String,
    /// 充电计划
    pub charging_schedule: ChargingScheduleType,
    /// EVSE ID
    pub evse_id: i32,
}

impl NotifyEVChargingScheduleRequest {
    pub fn new(time_base: impl Into<String>, charging_schedule: ChargingScheduleType, evse_id: i32) -> Self {
        Self {
            time_base: time_base.into(),
            charging_schedule,
            evse_id,
        }
    }
}

pub const ACTION: &str = "NotifyEVChargingSchedule";