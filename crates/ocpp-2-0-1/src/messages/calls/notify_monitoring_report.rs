//! NotifyMonitoringReport Request (Functional Block D)
//! 上报监控配置（分页）

use serde::{Deserialize, Serialize};
use crate::common::ComponentType;

/// 监控数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: crate::common::VariableType,
    /// 监控集合
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

/// 变量监控类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    /// ID
    pub id: i32,
    /// 事务标志
    pub transaction: bool,
    /// 监控值
    pub value: f64,
    /// 监控类型
    #[serde(rename = "type")]
    pub monitor_type: super::set_variable_monitoring::MonitorEnumType,
    /// 严重度 (0-9)
    pub severity: i32,
}

/// NotifyMonitoringReport 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 序列号
    pub seq_no: i32,
    /// 报告生成时间
    pub generated_at: String,
    /// 监控数据列表 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Vec<MonitoringDataType>>,
    /// 是否有后续数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

impl NotifyMonitoringReportRequest {
    pub fn new(request_id: i32, seq_no: i32, generated_at: impl Into<String>) -> Self {
        Self {
            request_id,
            seq_no,
            generated_at: generated_at.into(),
            monitor: None,
            tbc: Some(false),
        }
    }
}

pub const ACTION: &str = "NotifyMonitoringReport";