//! 充电枪/连接器 entity（对应 `charge_connector_ocpp16` 表）。
//!
//! 主键为复合主键 `(charge_point_id, connector_id)`，均无自增。
//! 业务上"一把枪是某桩下编号唯一"决定了此设计。
//!
//! 枪记录由 OCPP StatusNotification 触发创建（on-demand），无独立的 HTTP 创建端点。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::ConnectorType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_connector_ocpp16")]
pub struct Model {
    /// 所属充电桩唯一标识（复合主键第一段）
    #[sea_orm(primary_key, auto_increment = false)]
    pub charge_point_id: String,
    /// 枪编号（复合主键第二段，如 "1" / "2"）
    #[sea_orm(primary_key, auto_increment = false)]
    pub connector_id: String,
    /// 接口类型：1 国标直流，2 国标交流
    #[sea_orm(column_type = "SmallInteger")]
    pub connector_type: ConnectorType,
    /// 枪状态（VARCHAR 64，OCPP ChargePointStatus 字符串）
    pub status: String,
    /// 错误码
    pub error_code: Option<String>,
    /// 枪级状态最后更新时间
    #[sea_orm(column_type = "Timestamp")]
    pub last_heartbeat_time: Option<DateTime>,
    /// 记录创建时间
    #[sea_orm(column_type = "Timestamp")]
    pub create_time: DateTime,
    /// 记录更新时间
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
    #[sea_orm(has_many = "super::charge_transaction::Entity")]
    Transactions,
}

impl Related<super::charge_point::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChargePoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
