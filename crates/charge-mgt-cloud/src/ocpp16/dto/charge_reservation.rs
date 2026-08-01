//! 充电预约资源 DTO。

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::ocpp16::entity::enums::ReservationStatus;

/// `POST /api/v1/reservations` 请求体。
///
/// service 层校验 `end_time > start_time`（违反则 400）。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateReservation {
    pub user_id: i64,
    pub charge_point_id: String,
    pub connector_id: Option<String>,
    pub tag_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

/// `PATCH /api/v1/reservations/:id` 请求体。
///
/// 仅 `status == Pending` 时允许调用，否则 service 层返回 400。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReservation {
    pub connector_id: Option<String>,
    pub tag_id: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

/// `POST /api/v1/reservations/:id/cancel` 请求体。
#[derive(Debug, Clone, Deserialize)]
pub struct CancelReservation {
    pub cancel_reason: Option<String>,
}

/// `GET /api/v1/reservations` query string。
#[derive(Debug, Default, Deserialize)]
pub struct ReservationListQuery {
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub charge_point_id: Option<String>,
    #[serde(default)]
    pub status: Option<ReservationStatus>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ReservationListQuery {
    /// 转 [`super::common::PageQuery`]。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 预约响应体。
pub type ReservationResponse = crate::ocpp16::entity::charge_reservation::Model;
