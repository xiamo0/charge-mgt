//! ClearTariffs Confirmation (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::ClearTariffsResultType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsConfirmation {
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,
}
