//! GetBaseReport Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::ReportBaseEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    pub request_id: i32,
    pub report_base: ReportBaseEnumType,
}

pub const ACTION: &str = "GetBaseReport";
