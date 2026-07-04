//! GetReport Request (Functional Block B)
//! 请求过滤配置报告

use crate::common::{ComponentCriterionEnumType, ComponentVariableType};
use serde::{Deserialize, Serialize};

/// GetReport 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 组件标准 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,
    /// 组件变量 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
}

impl GetReportRequest {
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            component_criteria: None,
            component_variable: None,
        }
    }

    /// 添加组件标准
    pub fn with_criteria(mut self, criteria: Vec<ComponentCriterionEnumType>) -> Self {
        self.component_criteria = Some(criteria);
        self
    }

    /// 添加组件变量
    pub fn with_variables(mut self, variables: Vec<ComponentVariableType>) -> Self {
        self.component_variable = Some(variables);
        self
    }
}

pub const ACTION: &str = "GetReport";
