//! NotifyPriorityCharging Confirmation (Block K — 2.1)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyPriorityChargingConfirmation {}

impl NotifyPriorityChargingConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
