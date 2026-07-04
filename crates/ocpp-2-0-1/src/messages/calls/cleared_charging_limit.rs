//! ClearedChargingLimit Request (Functional Block A)
//! 充电限制清除通知

use serde::{Deserialize, Serialize};

/// ClearedChargingLimit 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    /// EVSE ID
    pub evse_id: i32,
}

impl ClearedChargingLimitRequest {
    pub fn new(evse_id: i32) -> Self {
        Self { evse_id }
    }
}

pub const ACTION: &str = "ClearedChargingLimit";
