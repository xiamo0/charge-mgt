//! SetVariableMonitoring Confirmation (Block N)
use serde::{Deserialize, Serialize};
use crate::common::SetMonitoringResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringConfirmation {
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}
