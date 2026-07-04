//! ClosePeriodicEventStream Request (Block N — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosePeriodicEventStreamRequest {
    pub id: i32,
}

pub const ACTION: &str = "ClosePeriodicEventStream";
