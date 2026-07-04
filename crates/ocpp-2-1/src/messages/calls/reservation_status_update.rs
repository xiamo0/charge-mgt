//! ReservationStatusUpdate Request (Block H)
use crate::common::ReservationUpdateStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    pub reservation_id: i32,
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

pub const ACTION: &str = "ReservationStatusUpdate";
