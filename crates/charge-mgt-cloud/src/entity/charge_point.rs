//! 充电桩设备 entity（对应 `charge_point_ocpp16` 表）。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_point_ocpp16")]
pub struct Model {
    /// 充电桩唯一标识（WS 请求地址中夹带），业务主键，无自增
    #[sea_orm(primary_key, auto_increment = false)]
    pub charge_point_id: String,
    /// 所属充电站 ID
    pub station_id: i64,
    pub charge_point_vendor: Option<String>,
    pub charge_point_model: Option<String>,
    pub charge_box_serial_number: Option<String>,
    pub charge_point_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    /// SIM 卡 ICCID
    pub iccid: Option<String>,
    /// SIM 卡 IMSI
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
    /// OCPP ChargePointStatus 枚举字符串（VARCHAR 64）
    pub status: String,
    /// OCPP ChargePointErrorCode 枚举字符串
    pub error_code: Option<String>,
    /// 安装投运日期
    #[sea_orm(column_type = "Date")]
    pub install_date: Option<Date>,
    /// 逻辑删除：0 正常，1 已删除
    pub is_deleted: i16,
    #[sea_orm(column_type = "Timestamp")]
    pub create_time: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::charge_connector::Entity")]
    Connectors,
    #[sea_orm(has_many = "super::charge_transaction::Entity")]
    Transactions,
    #[sea_orm(has_many = "super::charge_reservation::Entity")]
    Reservations,
    #[sea_orm(has_many = "super::smart_charge_profile::Entity")]
    Profiles,
}

impl Related<super::charge_connector::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connectors.def()
    }
}

impl Related<super::charge_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transactions.def()
    }
}

impl Related<super::charge_reservation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reservations.def()
    }
}

impl Related<super::smart_charge_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profiles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
