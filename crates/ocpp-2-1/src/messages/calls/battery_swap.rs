//! BatterySwap Request (Block S — 2.1 New)
use crate::common::{BatteryDataType, BatterySwapEventEnumType, IdTokenType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    pub id_token: IdTokenType,
    pub event_type: BatterySwapEventEnumType,
    pub request_id: i32,
    pub battery_data: Vec<BatteryDataType>,
}

pub const ACTION: &str = "BatterySwap";
