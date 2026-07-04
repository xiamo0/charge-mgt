//! ClosePeriodicEventStream Confirmation (Block N — 2.1)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClosePeriodicEventStreamConfirmation {}

impl ClosePeriodicEventStreamConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
