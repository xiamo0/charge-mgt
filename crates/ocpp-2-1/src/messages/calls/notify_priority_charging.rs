//! NotifyPriorityCharging Request (Block K — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPriorityChargingRequest {
    pub transaction_id: String,
    pub activated: bool,
}

pub const ACTION: &str = "NotifyPriorityCharging";
