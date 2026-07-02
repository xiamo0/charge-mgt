//! GetDisplayMessages Request (Functional Block L)
//! 查询已存储显示消息

use serde::{Deserialize, Serialize};
use super::set_display_message::MessagePriorityEnumType;
use super::set_display_message::MessageStateEnumType;

/// GetDisplayMessages 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 消息 ID 列表 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,
    /// 优先级 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,
    /// 状态 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
}

impl GetDisplayMessagesRequest {
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            id: None,
            priority: None,
            state: None,
        }
    }
}

pub const ACTION: &str = "GetDisplayMessages";