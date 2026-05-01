//! ClearChargingProfile 响应

use crate::common::status::ClearChargingProfileStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearChargingProfileConfirmation {
    pub status: ClearChargingProfileStatus,
}

impl ClearChargingProfileConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: ClearChargingProfileStatus::Accepted,
        }
    }
}
