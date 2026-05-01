//! CancelReservation 响应

use serde::{Deserialize, Serialize};
use crate::common::status::CancelReservationStatus;

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