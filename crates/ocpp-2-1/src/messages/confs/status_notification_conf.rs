//! StatusNotification Confirmation (Block G)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusNotificationConfirmation {}

impl StatusNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
