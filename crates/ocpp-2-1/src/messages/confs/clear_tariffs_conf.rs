//! ClearTariffs Confirmation (Block I — 2.1 New)
use crate::common::ClearTariffsResultType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsConfirmation {
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,
}
