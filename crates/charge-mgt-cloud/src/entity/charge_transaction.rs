//! 充电事务/订单核心 entity（对应 `charge_transaction_ocpp16` 表）。

use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::{PaymentStatus, TransactionStatus};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_transaction_ocpp16")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    /// OCPP 事务 ID（桩端生成，全局唯一），有 UNIQUE 索引
    pub transaction_id: String,
    pub user_id: Option<i64>,
    /// 触发充电的身份标签（RFID / Token）
    pub tag_id: String,
    /// 充电桩唯一标识
    pub charge_point_id: String,
    /// 充电枪编号
    pub connector_id: String,
    /// 事务状态：0 进行中、1 正常结束、2 异常中断、3 离线补传待处理
    #[sea_orm(column_type = "SmallInteger")]
    pub status: TransactionStatus,
    /// 结束原因（EVDisconnected、HardReset、Local、Remote、DeAuthorized 等）
    pub stop_reason: Option<String>,
    /// 充电开始时间（桩端上报）
    #[sea_orm(column_type = "Timestamp")]
    pub start_time: DateTime,
    /// 充电结束时间
    #[sea_orm(column_type = "Timestamp")]
    pub end_time: Option<DateTime>,
    /// 起始电表读数（kWh）
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    pub meter_start: Decimal,
    /// 结束电表读数（kWh）
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    pub meter_stop: Option<Decimal>,
    /// 总充电量（kWh）
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    pub total_energy: Option<Decimal>,
    /// 总费用（元）
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub total_amount: Option<Decimal>,
    /// 电费（元）
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub electricity_fee: Option<Decimal>,
    /// 服务费（元）
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub service_fee: Option<Decimal>,
    /// 支付状态：0 未支付、1 已支付、2 支付失败、3 已退款
    #[sea_orm(column_type = "SmallInteger")]
    pub payment_status: PaymentStatus,
    /// 是否为离线补传：0 实时上报、1 离线补传
    pub is_offline_sync: i16,
    /// 数据同步重试次数
    pub sync_attempts: Option<i32>,
    #[sea_orm(column_type = "Timestamp")]
    pub last_sync_time: Option<DateTime>,
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
    #[sea_orm(
        belongs_to = "super::charge_connector::Entity",
        from = "(Column::ChargePointId, Column::ConnectorId)",
        to = "(super::charge_connector::Column::ChargePointId, super::charge_connector::Column::ConnectorId)"
    )]
    Connector,
}

impl Related<super::charge_point::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChargePoint.def()
    }
}

impl Related<super::charge_connector::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connector.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
