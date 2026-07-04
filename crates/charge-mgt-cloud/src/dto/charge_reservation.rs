use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::entity::enums::ReservationStatus;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateReservation {
    pub user_id: i64,
    pub charge_point_id: String,
    pub connector_id: Option<String>,
    pub tag_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReservation {
    pub connector_id: Option<String>,
    pub tag_id: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelReservation {
    pub cancel_reason: Option<String>,
}

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
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type ReservationResponse = crate::entity::charge_reservation::Model;
