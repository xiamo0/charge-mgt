//! ClearDisplayMessage Request (Functional Block L)
//! 清除显示消息

use serde::{Deserialize, Serialize};

/// ClearDisplayMessage 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    /// 消息 ID
    pub id: i32,
}

impl ClearDisplayMessageRequest {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub const ACTION: &str = "ClearDisplayMessage";
