//! CancelReservation Request (Block H)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    pub reservation_id: i32,
}

pub const ACTION: &str = "CancelReservation";
