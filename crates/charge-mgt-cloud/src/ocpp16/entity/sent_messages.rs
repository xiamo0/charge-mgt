//! 已收到的 OCPP 报文幂等屏障 entity（对应 `charge_mgt_sent_messages_ocpp_1_6` 表）。
//!
//! 注意：本表是历史遗留的 OCPP 1.6 入站消息表，**不在 Phase 0 SQL 的 6 张业务表范围内**，
//! 由 `migration::m20250101_000000_phase0_init` 的 `down()` 显式 drop。
//! 字段命名也保留旧式（`gateway_id` / `direction` 等），与新版 6 张表不一致。
//!
//! 用途：
//! * `dispatcher` 在处理每条 OCPP 消息前先 INSERT 该记录，利用唯一键（`unique_id`）
//!   实现重复消息跳过（`ON CONFLICT DO NOTHING`）。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_mgt_sent_messages_ocpp_1_6")]
pub struct Model {
    /// OCPP 报文的 messageId，平台内全局唯一
    #[sea_orm(primary_key, auto_increment = false)]
    pub unique_id: String,
    /// 上游网关 ID（OCPP-J 走 Kafka 转发时的 producer 标识）
    pub gateway_id: String,
    /// 报文对应的充电桩 ID
    pub charge_point_id: String,
    /// 报文方向（如 `incoming` / `outgoing`）
    pub direction: String,
    /// OCPP Action 名（如 `BootNotification` / `Heartbeat`）
    pub action: String,
    /// OCPP 消息类型（如 `CALL` / `CALLRESULT` / `CALLERROR`）
    pub message_type: String,
    /// 报文接收时间（带时区）
    pub received_at: DateTimeWithTimeZone,
    /// 报文处理完成时间（带时区）
    pub processed_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
