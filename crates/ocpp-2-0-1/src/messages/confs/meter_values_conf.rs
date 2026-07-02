//! MeterValues Confirmation (Functional Block J)
//! 空 payload

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MeterValuesConfirmation {}

impl MeterValuesConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}