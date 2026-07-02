//! NotifyEvent Request (Functional Block D)
//! 上报变量监控事件

use serde::{Deserialize, Serialize};
use crate::common::{ComponentType, VariableType};

/// 事件触发类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}

/// 事件通知类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}

/// 事件数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    /// 事件 ID
    pub event_id: i32,
    /// 时间戳
    pub timestamp: String,
    /// 触发类型
    pub trigger: EventTriggerEnumType,
    /// 原因 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i32>,
    /// 实际值
    pub actual_value: String,
    /// 事务标志
    pub tech_code: Option<String>,
    /// 事务状态
    pub tech_info: Option<String>,
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 事件通知类型
    pub event_notification_type: EventNotificationEnumType,
}

/// NotifyEvent 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    /// 生成时间
    pub generated_at: String,
    /// 序列号
    pub seq_no: i32,
    /// 是否有后续数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    /// 事件数据列表
    pub event_data: Vec<EventDataType>,
}

impl NotifyEventRequest {
    pub fn new(generated_at: impl Into<String>, seq_no: i32, event_data: Vec<EventDataType>) -> Self {
        Self {
            generated_at: generated_at.into(),
            seq_no,
            tbc: Some(false),
            event_data,
        }
    }
}

pub const ACTION: &str = "NotifyEvent";