//! SendLocalList 响应

use serde::{Deserialize, Serialize};
use crate::common::status::UpdateStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendLocalListConfirmation {
    pub status: UpdateStatus,
}

impl SendLocalListConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: UpdateStatus::Accepted,
        }
    }
}