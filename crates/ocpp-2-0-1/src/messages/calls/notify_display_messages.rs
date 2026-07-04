//! NotifyDisplayMessages Request (Functional Block L)
//! 上报已存储显示消息（分页）

use super::set_display_message::MessageInfoType;
use serde::{Deserialize, Serialize};

/// NotifyDisplayMessages 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 是否有后续数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    /// 消息列表 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_info: Option<Vec<MessageInfoType>>,
}

impl NotifyDisplayMessagesRequest {
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            tbc: Some(false),
            message_info: None,
        }
    }

    pub fn with_messages(mut self, messages: Vec<MessageInfoType>) -> Self {
        self.message_info = Some(messages);
        self
    }
}

pub const ACTION: &str = "NotifyDisplayMessages";
