//! BatterySwap Request (Block S — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{BatteryDataType, BatterySwapEventEnumType, IdTokenType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    pub id_token: IdTokenType,
    pub event_type: BatterySwapEventEnumType,
    pub request_id: i32,
    pub battery_data: Vec<BatteryDataType>,
}

pub const ACTION: &str = "BatterySwap";
