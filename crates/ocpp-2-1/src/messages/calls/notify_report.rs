//! NotifyReport Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::ReportDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    pub request_id: i32,
    pub generated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_data: Option<Vec<ReportDataType>>,
}

pub const ACTION: &str = "NotifyReport";
