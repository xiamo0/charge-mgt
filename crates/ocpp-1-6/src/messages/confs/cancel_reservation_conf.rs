//! CancelReservation 响应

use crate::common::status::CancelReservationStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelReservationConfirmation {
    pub status: CancelReservationStatus,
}

impl CancelReservationConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: CancelReservationStatus::Accepted,
        }
    }
}
