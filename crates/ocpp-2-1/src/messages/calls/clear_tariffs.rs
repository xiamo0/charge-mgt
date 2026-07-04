//! ClearTariffs Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_ids: Option<Vec<String>>,
}

pub const ACTION: &str = "ClearTariffs";
