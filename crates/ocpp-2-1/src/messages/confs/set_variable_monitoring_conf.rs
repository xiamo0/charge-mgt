//! SetVariableMonitoring Confirmation (Block N)
use crate::common::SetMonitoringResultType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringConfirmation {
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}
