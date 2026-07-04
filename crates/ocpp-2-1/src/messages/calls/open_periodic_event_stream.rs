//! OpenPeriodicEventStream Request (Block N — 2.1 New)
use crate::common::ConstantStreamDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamRequest {
    pub constant_stream_data: ConstantStreamDataType,
}

pub const ACTION: &str = "OpenPeriodicEventStream";
