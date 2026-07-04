//! GetReport Request (Block B)
use crate::common::{ComponentCriterionEnumType, ComponentVariableType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,
}

pub const ACTION: &str = "GetReport";
