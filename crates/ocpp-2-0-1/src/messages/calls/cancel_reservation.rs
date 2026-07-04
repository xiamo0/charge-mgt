//! CancelReservation Request (Functional Block H)
//! 取消预约

use serde::{Deserialize, Serialize};

/// CancelReservation 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    /// 预约 ID
    pub reservation_id: i32,
}

impl CancelReservationRequest {
    pub fn new(reservation_id: i32) -> Self {
        Self { reservation_id }
    }
}

pub const ACTION: &str = "CancelReservation";
