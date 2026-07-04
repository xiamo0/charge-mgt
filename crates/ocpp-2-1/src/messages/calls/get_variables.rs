//! GetVariables Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::GetVariableDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    pub get_variable_data: Vec<GetVariableDataType>,
}

pub const ACTION: &str = "GetVariables";
