//! StatusNotification Confirmation (Functional Block F)
//! 空 payload

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusNotificationConfirmation {}

impl StatusNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}