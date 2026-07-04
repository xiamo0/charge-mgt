//! RequestBatterySwap Request (Block S — 2.1 New)
use crate::common::IdTokenType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBatterySwapRequest {
    pub id_token: IdTokenType,
    pub request_id: i32,
}

pub const ACTION: &str = "RequestBatterySwap";
