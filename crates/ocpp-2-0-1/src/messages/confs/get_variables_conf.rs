//! GetVariables Confirmation

use serde::{Deserialize, Serialize};
use crate::common::GetVariableResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesConfirmation {
    pub get_variable_result: Vec<GetVariableResultType>,
}

impl GetVariablesConfirmation {
    pub fn new(results: Vec<GetVariableResultType>) -> Self {
        Self {
            get_variable_result: results,
        }
    }
}