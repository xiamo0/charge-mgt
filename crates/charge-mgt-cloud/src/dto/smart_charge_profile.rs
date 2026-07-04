use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::entity::enums::ProfileDeliveryStatus;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProfile {
    pub charge_point_id: String,
    pub connector_id: Option<String>,
    pub charging_profile_id: i32,
    pub stack_level: i16,
    pub charging_profile_purpose: String,
    pub charging_profile_kind: String,
    pub start_time: Option<NaiveDateTime>,
    pub duration: Option<i32>,
    pub max_power_kw: Option<Decimal>,
    pub max_current_a: Option<Decimal>,
    #[serde(default = "default_pending")]
    pub status: ProfileDeliveryStatus,
}

fn default_pending() -> ProfileDeliveryStatus {
    ProfileDeliveryStatus::Pending
}

#[derive(Debug, Default, Deserialize)]
pub struct ProfileListQuery {
    #[serde(default)]
    pub charge_point_id: Option<String>,
    #[serde(default)]
    pub connector_id: Option<String>,
    #[serde(default)]
    pub charging_profile_purpose: Option<String>,
    #[serde(default)]
    pub status: Option<ProfileDeliveryStatus>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ProfileListQuery {
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type ProfileResponse = crate::entity::smart_charge_profile::Model;
