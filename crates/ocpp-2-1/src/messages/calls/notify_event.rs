//! NotifyEvent Request (Block N)
use crate::common::EventDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    pub generated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    pub event_data: Vec<EventDataType>,
}

pub const ACTION: &str = "NotifyEvent";
