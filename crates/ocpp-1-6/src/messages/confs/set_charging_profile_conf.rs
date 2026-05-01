//! SetChargingProfile 响应

use serde::{Deserialize, Serialize};
use crate::common::status::ChargingProfileStatus;

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