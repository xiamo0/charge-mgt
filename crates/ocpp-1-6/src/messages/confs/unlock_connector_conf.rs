//! UnlockConnector 响应

use crate::common::status::UnlockStatus;
use serde::{Deserialize, Serialize};

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
