//! TriggerMessage 响应

use crate::common::status::TriggerMessageStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerMessageConfirmation {
    pub status: TriggerMessageStatus,
}

impl TriggerMessageConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: TriggerMessageStatus::Accepted,
        }
    }
}
