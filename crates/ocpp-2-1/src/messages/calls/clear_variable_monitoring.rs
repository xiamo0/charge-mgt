//! ClearVariableMonitoring Request (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    pub id: Vec<i32>,
}

pub const ACTION: &str = "ClearVariableMonitoring";
