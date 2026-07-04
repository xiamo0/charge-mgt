//! NotifyDisplayMessages Request (Block O)
use crate::common::MessageInfoType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_info: Option<Vec<MessageInfoType>>,
}

pub const ACTION: &str = "NotifyDisplayMessages";
