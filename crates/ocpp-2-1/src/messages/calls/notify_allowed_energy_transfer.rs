//! NotifyAllowedEnergyTransfer Request (Block Q — 2.1 New)
use crate::common::EnergyTransferModeEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferRequest {
    pub transaction_id: String,
    pub allowed_energy_transfer: Vec<EnergyTransferModeEnumType>,
}

pub const ACTION: &str = "NotifyAllowedEnergyTransfer";
