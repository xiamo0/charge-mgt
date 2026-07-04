//! NotifyWebPaymentStarted Confirmation (Block I — 2.1)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyWebPaymentStartedConfirmation {}

impl NotifyWebPaymentStartedConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
