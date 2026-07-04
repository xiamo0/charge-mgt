//! TransactionEvent Request (Functional Block E)
//! 事务生命周期事件（替代 OCPP 1.6 的 StartTransaction/StopTransaction）

use crate::common::{
    EVSEType, IdTokenType, MeterValueType, TransactionEventEnumType, TransactionType,
    TriggerReasonEnumType,
};
use serde::{Deserialize, Serialize};

/// TransactionEvent 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    /// 事件类型: Started/Updated/Ended
    pub event_type: TransactionEventEnumType,
    /// 时间戳
    pub timestamp: String,
    /// 触发原因
    pub trigger_reason: TriggerReasonEnumType,
    /// 序列号 (递增)
    pub seq_no: i32,
    /// 事务信息
    pub transaction_info: TransactionType,
    /// 离线标志 (可选, 默认 false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    /// 使用相数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i32>,
    /// 线缆最大电流 (A) (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i32>,
    /// 关联预约 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    /// EVSE (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
    /// 认证标识 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    /// 计量值 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_value: Option<Vec<MeterValueType>>,
}

impl TransactionEventRequest {
    pub fn new(
        event_type: TransactionEventEnumType,
        timestamp: impl Into<String>,
        trigger_reason: TriggerReasonEnumType,
        seq_no: i32,
        transaction_info: TransactionType,
    ) -> Self {
        Self {
            event_type,
            timestamp: timestamp.into(),
            trigger_reason,
            seq_no,
            transaction_info,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            evse: None,
            id_token: None,
            meter_value: None,
        }
    }

    /// 创建事务开始事件
    pub fn started(timestamp: impl Into<String>, transaction_info: TransactionType) -> Self {
        Self::new(
            TransactionEventEnumType::Started,
            timestamp,
            TriggerReasonEnumType::Authorized,
            0,
            transaction_info,
        )
    }

    /// 创建事务更新事件
    pub fn updated(
        timestamp: impl Into<String>,
        trigger_reason: TriggerReasonEnumType,
        seq_no: i32,
        transaction_info: TransactionType,
    ) -> Self {
        Self::new(
            TransactionEventEnumType::Updated,
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info,
        )
    }

    /// 创建事务结束事件
    pub fn ended(
        timestamp: impl Into<String>,
        trigger_reason: TriggerReasonEnumType,
        seq_no: i32,
        transaction_info: TransactionType,
    ) -> Self {
        Self::new(
            TransactionEventEnumType::Ended,
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info,
        )
    }

    pub fn with_offline(mut self, offline: bool) -> Self {
        self.offline = Some(offline);
        self
    }

    pub fn with_number_of_phases_used(mut self, phases: i32) -> Self {
        self.number_of_phases_used = Some(phases);
        self
    }

    pub fn with_cable_max_current(mut self, current: i32) -> Self {
        self.cable_max_current = Some(current);
        self
    }

    pub fn with_reservation_id(mut self, id: i32) -> Self {
        self.reservation_id = Some(id);
        self
    }

    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }

    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    pub fn with_meter_value(mut self, meter_value: Vec<MeterValueType>) -> Self {
        self.meter_value = Some(meter_value);
        self
    }
}

pub const ACTION: &str = "TransactionEvent";
