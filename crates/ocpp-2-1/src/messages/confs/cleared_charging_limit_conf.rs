//! ClearedChargingLimit Confirmation (Block K)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClearedChargingLimitConfirmation {}

impl ClearedChargingLimitConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
