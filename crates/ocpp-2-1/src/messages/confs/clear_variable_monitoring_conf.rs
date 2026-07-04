//! ClearVariableMonitoring Confirmation (Block N)
use crate::common::ClearMonitoringResultType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringConfirmation {
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
}
