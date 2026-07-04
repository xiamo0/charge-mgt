//! NotifyMonitoringReport Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::MonitoringDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Vec<MonitoringDataType>>,
}

pub const ACTION: &str = "NotifyMonitoringReport";
