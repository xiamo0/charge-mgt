//! SetVariables Request (Block B)
use crate::common::SetVariableDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    pub set_variable_data: Vec<SetVariableDataType>,
}

pub const ACTION: &str = "SetVariables";
