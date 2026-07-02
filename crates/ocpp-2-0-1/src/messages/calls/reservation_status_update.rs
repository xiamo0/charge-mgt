//! ReservationStatusUpdate Request (Functional Block H)
//! 预约状态变更通知

use serde::{Deserialize, Serialize};

/// 预约状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
}

/// ReservationStatusUpdate 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    /// 预约 ID
    pub reservation_id: i32,
    /// 预约状态
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

impl ReservationStatusUpdateRequest {
    pub fn new(reservation_id: i32, status: ReservationUpdateStatusEnumType) -> Self {
        Self {
            reservation_id,
            reservation_update_status: status,
        }
    }
}

pub const ACTION: &str = "ReservationStatusUpdate";