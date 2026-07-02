//! SetVariables Confirmation

use serde::{Deserialize, Serialize};
use crate::common::SetVariableResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesConfirmation {
    pub set_variable_result: Vec<SetVariableResultType>,
}

impl SetVariablesConfirmation {
    pub fn new(results: Vec<SetVariableResultType>) -> Self {
        Self {
            set_variable_result: results,
        }
    }
}