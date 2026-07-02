//! SecurityEventNotification Confirmation (Functional Block A)
//! 空 payload

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventNotificationConfirmation {}

impl SecurityEventNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}