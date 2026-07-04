//! NotifyEVChargingNeeds Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::ChargingNeedsType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    pub evse_id: i32,
    pub charging_needs: ChargingNeedsType,
}

pub const ACTION: &str = "NotifyEVChargingNeeds";
