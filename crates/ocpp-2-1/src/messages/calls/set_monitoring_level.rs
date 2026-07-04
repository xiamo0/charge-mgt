//! SetMonitoringLevel Request (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    pub severity: i32,
}

pub const ACTION: &str = "SetMonitoringLevel";
