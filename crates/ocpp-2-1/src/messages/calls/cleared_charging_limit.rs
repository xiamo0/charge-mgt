//! ClearedChargingLimit Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::ChargingLimitSourceEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

pub const ACTION: &str = "ClearedChargingLimit";
