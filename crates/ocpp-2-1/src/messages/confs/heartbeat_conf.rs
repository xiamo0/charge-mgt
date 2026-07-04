//! Heartbeat Confirmation (Block B)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatConfirmation {
    pub current_time: String,
}

impl HeartbeatConfirmation {
    pub fn new(current_time: impl Into<String>) -> Self {
        Self { current_time: current_time.into() }
    }
    pub fn now() -> Self {
        Self::new(crate::common::datetime::now_rfc3339())
    }
}
