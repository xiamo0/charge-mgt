//! OCPP 2.0.1 / 2.1 Display management types (Functional Block O)

use crate::common::{ComponentType, MessageContentType};
use serde::{Deserialize, Serialize};

/// 显示消息优先级枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}

/// 显示消息状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}

/// 消息信息类型 (SetDisplayMessage/GetDisplayMessages/NotifyDisplayMessages)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    pub id: i32,
    pub priority: MessagePriorityEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub message: MessageContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,
}
