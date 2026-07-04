//! NotifyDERStartStop Request (Block R — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopRequest {
    pub control_id: String,
    pub started: bool,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superseded_ids: Option<Vec<String>>,
}

pub const ACTION: &str = "NotifyDERStartStop";
