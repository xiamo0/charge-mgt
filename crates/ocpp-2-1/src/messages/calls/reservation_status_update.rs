//! ReservationStatusUpdate Request (Block H)
use serde::{Deserialize, Serialize};
use crate::common::ReservationUpdateStatusEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    pub reservation_id: i32,
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

pub const ACTION: &str = "ReservationStatusUpdate";
