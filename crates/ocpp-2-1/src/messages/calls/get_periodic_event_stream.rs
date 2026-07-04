//! GetPeriodicEventStream Request (Block N — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetPeriodicEventStreamRequest {}

pub const ACTION: &str = "GetPeriodicEventStream";
