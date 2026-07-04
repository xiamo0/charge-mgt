//! ClearVariableMonitoring Confirmation (Block N)
use serde::{Deserialize, Serialize};
use crate::common::ClearMonitoringResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringConfirmation {
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
}
