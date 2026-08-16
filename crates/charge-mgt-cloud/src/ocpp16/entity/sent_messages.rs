//! 已收到的 OCPP 报文幂等屏障 entity（对应 `charge_mgt_sent_messages_ocpp_1_6` 表）。
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
