//! CostUpdated Confirmation
//! 空 payload

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostUpdatedConfirmation {}

impl CostUpdatedConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}