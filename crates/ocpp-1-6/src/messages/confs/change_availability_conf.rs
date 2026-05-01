//! ChangeAvailability 响应

use crate::common::status::AvailabilityStatus;
use serde::{Deserialize, Serialize};

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
