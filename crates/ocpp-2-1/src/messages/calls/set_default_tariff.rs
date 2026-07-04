//! SetDefaultTariff Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::TariffType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultTariffRequest {
    pub evse_id: i32,
    pub tariff: TariffType,
}

pub const ACTION: &str = "SetDefaultTariff";
