//! ChangeAvailability 响应

use serde::{Deserialize, Serialize};
use crate::common::status::AvailabilityStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeAvailabilityConfirmation {
    pub status: AvailabilityStatus,
}

impl ChangeAvailabilityConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: AvailabilityStatus::Accepted,
        }
    }
}