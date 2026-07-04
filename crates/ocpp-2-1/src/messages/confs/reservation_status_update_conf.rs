//! ReservationStatusUpdate Confirmation (Block H)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationStatusUpdateConfirmation {}

impl ReservationStatusUpdateConfirmation {
    pub fn new() -> Self {
        Self {}
    }
}
