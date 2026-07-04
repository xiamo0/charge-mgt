//! PublishFirmwareStatusNotification Confirmation (Block L)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishFirmwareStatusNotificationConfirmation {}

impl PublishFirmwareStatusNotificationConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
