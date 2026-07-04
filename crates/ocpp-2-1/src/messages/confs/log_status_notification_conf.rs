//! LogStatusNotification Confirmation (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogStatusNotificationConfirmation {}

impl LogStatusNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
