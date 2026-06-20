use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_mgt_charge_points_ocpp_1_6")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub gateway_id: String,
    pub gateway_ip: Option<String>,
    pub vendor: String,
    pub model: String,
    pub serial_number: Option<String>,
    pub charge_box_serial: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
    pub protocol_version: String,
    pub ocpp_status: String,
    pub heartbeat_interval_secs: i32,
    pub last_heartbeat_at: Option<DateTimeWithTimeZone>,
    pub last_boot_at: Option<DateTimeWithTimeZone>,
    pub registered_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
