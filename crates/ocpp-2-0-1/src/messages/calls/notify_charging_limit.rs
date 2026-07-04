//! NotifyChargingLimit Request (Functional Block A)
//! 上报外部充电限制

use serde::{Deserialize, Serialize};

/// NotifyChargingLimit 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    /// 充电限制
    pub charging_limit: ChargingLimitType,
    /// 充电曲线 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<Vec<crate::common::ChargingScheduleType>>,
    /// EVSE ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

/// 充电限制类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    /// 限制来源
    pub charging_limit_source: super::report_charging_profiles::ChargingLimitSourceEnumType,
    /// 是否可网格关键
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,
}

impl NotifyChargingLimitRequest {
    pub fn new(charging_limit: ChargingLimitType) -> Self {
        Self {
            charging_limit,
            charging_schedule: None,
            evse_id: None,
        }
    }

    /// 添加充电曲线
    pub fn with_charging_schedule(
        mut self,
        schedule: Vec<crate::common::ChargingScheduleType>,
    ) -> Self {
        self.charging_schedule = Some(schedule);
        self
    }

    /// 设置 EVSE ID
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }
}

pub const ACTION: &str = "NotifyChargingLimit";
