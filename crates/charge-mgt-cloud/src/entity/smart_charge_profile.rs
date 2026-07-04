//! 智能充电策略 entity（对应 `smart_charge_profile_ocpp16` 表）。

use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::ProfileDeliveryStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "smart_charge_profile_ocpp16")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub charge_point_id: String,
    /// 充电枪编号；空 / "0" 代表整个桩
    pub connector_id: Option<String>,
    /// 桩端生成的 Profile 唯一 ID（非本表主键）
    pub charging_profile_id: i32,
    /// 策略优先级（数字越大越高）
    pub stack_level: i16,
    /// 策略目的：ChargePointMaxProfile / TxDefaultProfile / TxProfile
    pub charging_profile_purpose: String,
    /// 策略类型：Absolute / Recurring / Relative
    pub charging_profile_kind: String,
    /// 计划生效时间
    #[sea_orm(column_type = "Timestamp")]
    pub start_time: Option<DateTime>,
    /// 计划持续时间（秒）
    pub duration: Option<i32>,
    /// 限制的最大功率（kW）
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    pub max_power_kw: Option<Decimal>,
    /// 限制的最大电流（A）
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    pub max_current_a: Option<Decimal>,
    /// 下发状态：0 待下发、1 已接受、2 已拒绝
    #[sea_orm(column_type = "SmallInteger")]
    pub status: ProfileDeliveryStatus,
    #[sea_orm(column_type = "Timestamp")]
    pub create_time: DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::charge_point::Entity",
        from = "Column::ChargePointId",
        to = "super::charge_point::Column::ChargePointId"
    )]
    ChargePoint,
}

impl Related<super::charge_point::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChargePoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
