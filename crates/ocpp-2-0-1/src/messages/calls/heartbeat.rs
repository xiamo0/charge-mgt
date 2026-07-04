//! Heartbeat Request (Functional Block B)
//! 心跳保活

use serde::{Deserialize, Serialize};

/// Heartbeat 请求 (空 payload)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatRequest {}

impl HeartbeatRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for HeartbeatRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "Heartbeat";
