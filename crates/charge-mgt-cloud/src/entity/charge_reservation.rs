//! 充电预约 entity（对应 `charge_reservation_ocpp16` 表）。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::ReservationStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_reservation_ocpp16")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub reservation_id: i64,
    pub user_id: i64,
    pub charge_point_id: String,
    /// 关联充电枪编号（可选）
    pub connector_id: Option<String>,
    /// 预约时绑定的 RFID / Token（到达后直接启动）
    pub tag_id: Option<String>,
    /// 预约开始时间
    #[sea_orm(column_type = "Timestamp")]
    pub start_time: DateTime,
    /// 预约结束时间
    #[sea_orm(column_type = "Timestamp")]
    pub end_time: DateTime,
    /// 预约状态：0 待履约、1 进行中、2 已完成、3 已取消、4 已违约
    #[sea_orm(column_type = "SmallInteger")]
    pub status: ReservationStatus,
    /// 实际充电事务 ID（充电启动后回写）
    pub transaction_id: Option<String>,
    pub cancel_reason: Option<String>,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<DateTime>,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: Option<DateTime>,
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
