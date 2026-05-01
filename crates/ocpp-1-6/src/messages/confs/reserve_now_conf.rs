//! ReserveNow 响应

use crate::common::status::ReservationStatus;
use serde::{Deserialize, Serialize};

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
