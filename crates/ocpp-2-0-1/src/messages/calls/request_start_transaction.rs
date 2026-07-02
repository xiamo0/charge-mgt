//! RequestStartTransaction Request (Functional Block F)
//! 远程启动充电

use serde::{Deserialize, Serialize};
use crate::common::{IdTokenType, ChargingProfileType};

/// RequestStartTransaction 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    /// 远程启动 ID
    pub remote_start_id: i32,
    /// ID Token
    pub id_token: IdTokenType,
    /// EVSE ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// 充电配置 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType>,
}

impl RequestStartTransactionRequest {
    pub fn new(remote_start_id: i32, id_token: IdTokenType) -> Self {
        Self {
            remote_start_id,
            id_token,
            evse_id: None,
            charging_profile: None,
        }
    }

    /// 指定 EVSE
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }

    /// 设置充电配置
    pub fn with_charging_profile(mut self, profile: ChargingProfileType) -> Self {
        self.charging_profile = Some(profile);
        self
    }
}

pub const ACTION: &str = "RequestStartTransaction";