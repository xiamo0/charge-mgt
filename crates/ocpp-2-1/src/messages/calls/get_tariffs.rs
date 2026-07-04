//! GetTariffs Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsRequest {
    pub evse_id: i32,
}

pub const ACTION: &str = "GetTariffs";
