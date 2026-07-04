//! AFRRSignal Request (Block Q — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalRequest {
    pub signal: i32,
    pub timestamp: String,
}

pub const ACTION: &str = "AFRRSignal";
