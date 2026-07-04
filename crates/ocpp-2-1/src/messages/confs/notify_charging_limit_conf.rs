//! NotifyChargingLimit Confirmation (Block K)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyChargingLimitConfirmation {}

impl NotifyChargingLimitConfirmation {
    pub fn new() -> Self { Self {} }
}
