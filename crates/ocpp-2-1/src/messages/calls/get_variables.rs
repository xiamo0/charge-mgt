//! GetVariables Request (Block B)
use crate::common::GetVariableDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    pub get_variable_data: Vec<GetVariableDataType>,
}

pub const ACTION: &str = "GetVariables";
