//! TriggerMessage 响应

use serde::{Deserialize, Serialize};
use crate::common::status::TriggerMessageStatus;

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