//! SetVariableMonitoring Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::SetMonitoringDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

pub const ACTION: &str = "SetVariableMonitoring";
