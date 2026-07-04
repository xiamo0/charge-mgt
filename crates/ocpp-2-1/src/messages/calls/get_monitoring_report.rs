//! GetMonitoringReport Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::{ComponentVariableType, MonitoringCriterionEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_monitoring_data: Option<Vec<ComponentVariableType>>,
}

pub const ACTION: &str = "GetMonitoringReport";
