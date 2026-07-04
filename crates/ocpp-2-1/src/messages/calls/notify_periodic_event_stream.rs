//! NotifyPeriodicEventStream Request (Block N — 2.1 New)
//! 使用 SEND (MessageTypeId=6) 单向发送，无响应
use serde::{Deserialize, Serialize};
use crate::common::StreamDataElementType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyPeriodicEventStreamRequest {
    pub id: i32,
    pub basetime: String,
    pub pending: i32,
    pub data: Vec<StreamDataElementType>,
}

pub const ACTION: &str = "NotifyPeriodicEventStream";
