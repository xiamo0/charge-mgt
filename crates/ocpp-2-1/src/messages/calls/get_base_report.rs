//! GetBaseReport Request (Block B)
use crate::common::ReportBaseEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    pub request_id: i32,
    pub report_base: ReportBaseEnumType,
}

pub const ACTION: &str = "GetBaseReport";
