//! GetMonitoringReport Request (Functional Block D)
//! 请求监控配置报告

use crate::common::ComponentVariableType;
use serde::{Deserialize, Serialize};

/// 监控标准枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringCriterionEnumType {
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}

/// GetMonitoringReport 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 监控标准 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    /// 组件变量 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
}

impl GetMonitoringReportRequest {
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            monitoring_criteria: None,
            component_variable: None,
        }
    }

    pub fn with_criteria(mut self, criteria: Vec<MonitoringCriterionEnumType>) -> Self {
        self.monitoring_criteria = Some(criteria);
        self
    }

    pub fn with_variables(mut self, variables: Vec<ComponentVariableType>) -> Self {
        self.component_variable = Some(variables);
        self
    }
}

pub const ACTION: &str = "GetMonitoringReport";
