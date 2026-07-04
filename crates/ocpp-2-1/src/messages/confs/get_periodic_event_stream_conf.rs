//! GetPeriodicEventStream Confirmation (Block N — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::ConstantStreamDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_stream_data: Option<Vec<ConstantStreamDataType>>,
}
