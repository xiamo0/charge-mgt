//! Heartbeat Request (Block B)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HeartbeatRequest {}

pub const ACTION: &str = "Heartbeat";
