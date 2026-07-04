//! GetVariables Confirmation (Block B)
use crate::common::GetVariableResultType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesConfirmation {
    pub get_variable_result: Vec<GetVariableResultType>,
}
