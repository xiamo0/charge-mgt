//! NotifyReport Request (Functional Block B)
//! 上报配置报告（分页）

use crate::common::ReportDataType;
use serde::{Deserialize, Serialize};

/// NotifyReport 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    /// 请求 ID (对应 GetBaseReport 或 GetReport)
    pub request_id: i32,
    /// 报告生成时间
    pub generated_at: String,
    /// 序列号 (起始 0)
    pub seq_no: i32,
    /// 报告数据 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_data: Option<Vec<ReportDataType>>,
    /// 是否有后续数据 (To Be Continued)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

impl NotifyReportRequest {
    pub fn new(request_id: i32, generated_at: impl Into<String>, seq_no: i32) -> Self {
        Self {
            request_id,
            generated_at: generated_at.into(),
            seq_no,
            report_data: None,
            tbc: Some(false),
        }
    }

    /// 添加报告数据
    pub fn with_data(mut self, data: Vec<ReportDataType>) -> Self {
        self.report_data = Some(data);
        self
    }

    /// 标记还有后续数据
    pub fn to_be_continued(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
        self
    }
}

pub const ACTION: &str = "NotifyReport";
