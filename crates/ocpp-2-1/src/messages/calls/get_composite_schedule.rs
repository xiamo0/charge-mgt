//! GetCompositeSchedule Request (Block K)
use crate::common::ChargingRateUnitEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    pub evse_id: i32,
}

pub const ACTION: &str = "GetCompositeSchedule";
