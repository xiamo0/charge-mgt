//! UsePriorityCharging Request (Block K — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingRequest {
    pub transaction_id: String,
    pub activate: bool,
}

pub const ACTION: &str = "UsePriorityCharging";
