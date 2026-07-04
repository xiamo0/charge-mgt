//! PullDynamicScheduleUpdate Request (Block K — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateRequest {
    pub charging_profile_id: i32,
}

pub const ACTION: &str = "PullDynamicScheduleUpdate";
