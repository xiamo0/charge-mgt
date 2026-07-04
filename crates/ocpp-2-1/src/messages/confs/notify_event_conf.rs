//! NotifyEvent Confirmation (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyEventConfirmation {}

impl NotifyEventConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
