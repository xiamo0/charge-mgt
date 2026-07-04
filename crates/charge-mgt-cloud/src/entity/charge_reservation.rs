//! 充电预约 entity（对应 `charge_reservation_ocpp16` 表）。
//!
//! 状态机见 [`super::enums::ReservationStatus`]：一旦离开 `Pending` 不可再改回，
//! 强制约束在 [`crate::service::reservation::update`] 与
//! [`crate::service::reservation::cancel`] 中执行。
//!
//! 主键名为 `reservation_id`（非通用 `id`），迁移自早期 schema 命名约定。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::ReservationStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_reservation_ocpp16")]
pub struct Model {
    /// 预约主键（自增；DB 列名 `reservation_id`）
    #[sea_orm(primary_key)]
    pub reservation_id: i64,
    /// 预约用户 ID
    pub user_id: i64,
    /// 目标充电桩唯一标识
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
    /// 实际充电事务 ID（充电启动后由 OCPP 流程回写）
    pub transaction_id: Option<String>,
    /// 取消原因（用户主动取消或系统超时释放）
    pub cancel_reason: Option<String>,
    /// 记录创建时间
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: Option<DateTime>,
    /// 记录更新时间
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
