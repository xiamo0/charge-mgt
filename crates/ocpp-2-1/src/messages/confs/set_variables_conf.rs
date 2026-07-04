//! SetVariables Confirmation (Block B)
use serde::{Deserialize, Serialize};
use crate::common::SetVariableResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesConfirmation {
    pub set_variable_result: Vec<SetVariableResultType>,
}
