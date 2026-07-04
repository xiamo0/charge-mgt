//! SecurityEventNotification Confirmation (Block A)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityEventNotificationConfirmation {}

impl SecurityEventNotificationConfirmation {
    pub fn new() -> Self { Self {} }
}
