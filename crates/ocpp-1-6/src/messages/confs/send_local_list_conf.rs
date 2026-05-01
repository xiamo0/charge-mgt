//! SendLocalList 响应

use crate::common::status::UpdateStatus;
use serde::{Deserialize, Serialize};

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
