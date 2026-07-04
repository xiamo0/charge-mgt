//! SetVariableMonitoring Request (Block N)
use crate::common::SetMonitoringDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

pub const ACTION: &str = "SetVariableMonitoring";
