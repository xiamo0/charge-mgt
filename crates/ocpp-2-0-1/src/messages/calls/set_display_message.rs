//! SetDisplayMessage Request (Functional Block L)
//! 设置显示消息

use serde::{Deserialize, Serialize};

/// 消息状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageStateEnumType {
    Charging,
    Faulted,
    Idle,
    Unavailable,
}

/// 消息优先级枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessagePriorityEnumType {
    AlwaysFront,
    InFront,
    NormalCycle,
}

/// 显示消息类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// 消息 ID
    pub id: i32,
    /// 优先级
    pub priority: MessagePriorityEnumType,
    /// 消息状态 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
    /// 开始时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    /// 结束时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    /// 事务 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 消息内容
    pub message: crate::common::MessageContentType,
    /// 显示消息内容 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

/// SetDisplayMessage 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest {
    /// 消息信息
    pub message: MessageInfoType,
}

impl SetDisplayMessageRequest {
    pub fn new(message: MessageInfoType) -> Self {
        Self { message }
    }
}

pub const ACTION: &str = "SetDisplayMessage";