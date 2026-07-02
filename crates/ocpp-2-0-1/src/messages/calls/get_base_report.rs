//! GetBaseReport Request (Functional Block B)
//! 请求完整配置报告

use serde::{Deserialize, Serialize};
use crate::common::ReportBaseEnumType;

/// GetBaseReport 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 报告基础类型: ConfigurationInventory/FullInventory/SummaryInventory
    pub report_base: ReportBaseEnumType,
}

impl GetBaseReportRequest {
    pub fn new(request_id: i32, report_base: ReportBaseEnumType) -> Self {
        Self {
            request_id,
            report_base,
        }
    }
}

pub const ACTION: &str = "GetBaseReport";