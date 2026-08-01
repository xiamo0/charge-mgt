//! 智能充电策略资源 DTO。

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::ocpp16::entity::enums::ProfileDeliveryStatus;

/// `POST /api/v1/charging-profiles` 请求体。
///
/// 新策略创建时默认 `status == Pending`，等待 OCPP SetChargingProfile 回写为
/// `Accepted` / `Rejected`。
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
    /// 默认 [`ProfileDeliveryStatus::Pending`]
    #[serde(default = "default_pending")]
    pub status: ProfileDeliveryStatus,
}

fn default_pending() -> ProfileDeliveryStatus {
    ProfileDeliveryStatus::Pending
}

/// `GET /api/v1/charging-profiles` query string。
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
    /// 转 [`super::common::PageQuery`]。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 充电策略响应体。
pub type ProfileResponse = crate::ocpp16::entity::smart_charge_profile::Model;
