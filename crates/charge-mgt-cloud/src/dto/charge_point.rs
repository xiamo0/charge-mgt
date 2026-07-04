use chrono::NaiveDate;
use serde::Deserialize;

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

#[derive(Debug, Default, Deserialize)]
pub struct ChargePointListQuery {
    #[serde(default)]
    pub station_id: Option<i64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub include_deleted: Option<bool>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ChargePointListQuery {
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type ChargePointResponse = crate::entity::charge_point::Model;
