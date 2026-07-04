//! GetCompositeSchedule Request (Functional Block A)
//! 获取复合充电计划

use crate::common::ChargingRateUnitEnumType;
use serde::{Deserialize, Serialize};

/// GetCompositeSchedule 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// 时间范围 (秒)
    pub duration: i32,
    /// EVSE ID (>0)
    pub evse_id: i32,
    /// 充电速率单位 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
}

impl GetCompositeScheduleRequest {
    pub fn new(duration: i32, evse_id: i32) -> Self {
        Self {
            duration,
            evse_id,
            charging_rate_unit: None,
        }
    }

    /// 设置充电速率单位
    pub fn with_charging_rate_unit(mut self, unit: ChargingRateUnitEnumType) -> Self {
        self.charging_rate_unit = Some(unit);
        self
    }
}

pub const ACTION: &str = "GetCompositeSchedule";
