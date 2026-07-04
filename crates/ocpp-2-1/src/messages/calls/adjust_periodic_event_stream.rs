//! AdjustPeriodicEventStream Request (Block N — 2.1 New)
use crate::common::PeriodicEventStreamParamsType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamRequest {
    pub id: i32,
    pub params: PeriodicEventStreamParamsType,
}

pub const ACTION: &str = "AdjustPeriodicEventStream";
