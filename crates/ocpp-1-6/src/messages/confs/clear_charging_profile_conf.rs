//! ClearChargingProfile 响应

use serde::{Deserialize, Serialize};
use crate::common::status::ClearChargingProfileStatus;

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