//! UnlockConnector 响应

use serde::{Deserialize, Serialize};
use crate::common::status::UnlockStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnlockConnectorConfirmation {
    pub status: UnlockStatus,
}

impl UnlockConnectorConfirmation {
    pub fn unlocked() -> Self {
        Self {
            status: UnlockStatus::Unlocked,
        }
    }
}