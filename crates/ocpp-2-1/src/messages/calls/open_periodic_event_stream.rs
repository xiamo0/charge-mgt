//! OpenPeriodicEventStream Request (Block N — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::ConstantStreamDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamRequest {
    pub constant_stream_data: ConstantStreamDataType,
}

pub const ACTION: &str = "OpenPeriodicEventStream";
