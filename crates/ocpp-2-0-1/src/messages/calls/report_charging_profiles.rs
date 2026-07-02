//! ReportChargingProfiles Request (Functional Block A)
//! 上报充电曲线（分页）

use serde::{Deserialize, Serialize};
use crate::common::ChargingProfileType;

/// ReportChargingProfiles 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 充电曲线列表
    pub charging_limit: ChargingLimitSourceEnumType,
    /// 充电曲线
    pub charging_profile: Vec<ChargingProfileType>,
    /// 是否有后续数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

/// 充电限制来源
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingLimitSourceEnumType {
    EMS,
    Other,
    SO,
    CSO,
}

impl ReportChargingProfilesRequest {
    pub fn new(
        request_id: i32,
        charging_limit: ChargingLimitSourceEnumType,
        charging_profile: Vec<ChargingProfileType>,
    ) -> Self {
        Self {
            request_id,
            charging_limit,
            charging_profile,
            tbc: Some(false),
        }
    }

    /// 标记还有后续数据
    pub fn to_be_continued(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
        self
    }
}

pub const ACTION: &str = "ReportChargingProfiles";