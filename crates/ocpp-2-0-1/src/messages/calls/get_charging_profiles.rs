//! GetChargingProfiles Request (Functional Block A)
//! 获取已安装充电曲线

use serde::{Deserialize, Serialize};

/// GetChargingProfiles 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 充电曲线标准
    pub charging_profile: GetChargingProfileCriterion,
    /// EVSE ID (可选, 0 或不指定表示整个站点)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

/// 充电曲线查询标准
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfileCriterion {
    /// 充电曲线用途 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<super::super::calls::set_charging_profile::SetChargingProfileRequest>,
    /// 栈级别 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

impl GetChargingProfilesRequest {
    pub fn new(request_id: i32, charging_profile: GetChargingProfileCriterion) -> Self {
        Self {
            request_id,
            charging_profile,
            evse_id: None,
        }
    }

    /// 设置 EVSE ID
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }
}

pub const ACTION: &str = "GetChargingProfiles";