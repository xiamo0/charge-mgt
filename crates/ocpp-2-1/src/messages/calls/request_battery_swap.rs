//! RequestBatterySwap Request (Block S — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::IdTokenType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBatterySwapRequest {
    pub id_token: IdTokenType,
    pub request_id: i32,
}

pub const ACTION: &str = "RequestBatterySwap";
