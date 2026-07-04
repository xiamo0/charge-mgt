//! FirmwareStatusNotification Confirmation (Block L)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirmwareStatusNotificationConfirmation {}

impl FirmwareStatusNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
