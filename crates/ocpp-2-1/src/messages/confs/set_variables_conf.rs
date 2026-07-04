//! SetVariables Confirmation (Block B)
use crate::common::SetVariableResultType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesConfirmation {
    pub set_variable_result: Vec<SetVariableResultType>,
}
