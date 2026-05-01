//! SetChargingProfile 响应

use crate::common::status::ChargingProfileStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetChargingProfileConfirmation {
    pub status: ChargingProfileStatus,
}

impl SetChargingProfileConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: ChargingProfileStatus::Accepted,
        }
    }
}
