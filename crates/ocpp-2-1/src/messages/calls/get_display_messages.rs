//! GetDisplayMessages Request (Block O)
use crate::common::{MessagePriorityEnumType, MessageStateEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,
}

pub const ACTION: &str = "GetDisplayMessages";
