//! NotifyChargingLimit Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{ChargingLimitType, ChargingScheduleType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    pub charging_limit: ChargingLimitType,
    pub charging_schedule: Vec<ChargingScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

pub const ACTION: &str = "NotifyChargingLimit";
