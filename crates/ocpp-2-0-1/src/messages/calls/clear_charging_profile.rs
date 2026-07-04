//! ClearChargingProfile Request (Functional Block A)
//! 清除充电曲线

use crate::common::ChargingProfilePurposeEnumType;
use serde::{Deserialize, Serialize};

/// ClearChargingProfile 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// 曲线 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i32>,
    /// 曲线标准 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileTypeEnumType>,
}

/// 清除充电曲线标准
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileTypeEnumType {
    /// EVSE ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// 充电曲线用途 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    /// 栈级别 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

impl ClearChargingProfileRequest {
    pub fn new() -> Self {
        Self {
            charging_profile_id: None,
            charging_profile_criteria: None,
        }
    }

    /// 指定特定的曲线 ID
    pub fn with_profile_id(mut self, profile_id: i32) -> Self {
        self.charging_profile_id = Some(profile_id);
        self
    }

    /// 设置清除标准
    pub fn with_criteria(mut self, criteria: ClearChargingProfileTypeEnumType) -> Self {
        self.charging_profile_criteria = Some(criteria);
        self
    }
}

impl Default for ClearChargingProfileRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "ClearChargingProfile";
