//! ChangeTransactionTariff Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::TariffType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffRequest {
    pub transaction_id: String,
    pub tariff: TariffType,
}

pub const ACTION: &str = "ChangeTransactionTariff";
