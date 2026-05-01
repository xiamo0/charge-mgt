//! ReserveNow 响应

use serde::{Deserialize, Serialize};
use crate::common::status::ReservationStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReserveNowConfirmation {
    pub status: ReservationStatus,
}

impl ReserveNowConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: ReservationStatus::Accepted,
        }
    }
}