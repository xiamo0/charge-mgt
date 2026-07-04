//! 充电桩资源 DTO。

use chrono::NaiveDate;
use serde::Deserialize;

/// `POST /api/v1/charge-points` 请求体。
///
/// `charge_point_id` 是业务主键，重复创建会被 service 层返回 409。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateChargePoint {
    pub charge_point_id: String,
    pub station_id: i64,
    pub charge_point_vendor: Option<String>,
    pub charge_point_model: Option<String>,
    pub charge_box_serial_number: Option<String>,
    pub charge_point_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
    /// OCPP ChargePointStatus 字符串
    pub status: String,
    pub error_code: Option<String>,
    pub install_date: Option<NaiveDate>,
}

/// `PATCH /api/v1/charge-points/:id` 请求体。
///
/// 所有字段均为 `Option`，未提供的字段保持原值不变。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChargePoint {
    pub station_id: Option<i64>,
    pub charge_point_vendor: Option<String>,
    pub charge_point_model: Option<String>,
    pub charge_box_serial_number: Option<String>,
    pub charge_point_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
    pub status: Option<String>,
    pub error_code: Option<String>,
    pub install_date: Option<NaiveDate>,
}

/// `GET /api/v1/charge-points` query string。
///
/// 所有筛选条件均 `Option`，未提供则不过滤。
#[derive(Debug, Default, Deserialize)]
pub struct ChargePointListQuery {
    /// 按 `station_id` 过滤
    #[serde(default)]
    pub station_id: Option<i64>,
    /// 按 OCPP 状态字符串过滤（如 `Available` / `Faulted`）
    #[serde(default)]
    pub status: Option<String>,
    /// 是否包含软删除记录；默认 `false`（隐藏已软删）
    #[serde(default)]
    pub include_deleted: Option<bool>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ChargePointListQuery {
    /// 转 [`super::common::PageQuery`]，应用分页参数 normalize。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 充电桩响应体（直接复用 entity Model）。
pub type ChargePointResponse = crate::entity::charge_point::Model;
