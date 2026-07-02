//! Reset Request (Functional Block B)
//! 重启充电桩

use serde::{Deserialize, Serialize};
use crate::common::ResetEnumType;

/// Reset 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    /// 重启类型: Immediate/OnIdle
    #[serde(rename = "type")]
    pub reset_type: ResetEnumType,
    /// EVSE ID (可选, 指定特定 EVSE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

impl ResetRequest {
    pub fn new(reset_type: ResetEnumType) -> Self {
        Self {
            reset_type,
            evse_id: None,
        }
    }

    /// 指定 EVSE
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }
}

pub const ACTION: &str = "Reset";