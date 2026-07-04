//! SetVariables Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::SetVariableDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    pub set_variable_data: Vec<SetVariableDataType>,
}

pub const ACTION: &str = "SetVariables";
