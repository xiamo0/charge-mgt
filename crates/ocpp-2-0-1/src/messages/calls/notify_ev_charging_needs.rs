//! NotifyEVChargingNeeds Request (Functional Block B - ISO 15118)
//! 转发 EV 充电需求

use serde::{Deserialize, Serialize};

/// NotifyEVChargingNeeds 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    /// EVSE ID
    pub evse_id: i32,
    /// 充电需求
    pub charging_needs: ChargingNeedsType,
    /// 最大调度周期 (可选, 秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_schedule_tuples: Option<i32>,
}

/// 充电需求类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// 请求能量转移模式
    pub requested_energy_transfer: EnergyTransferModeEnumType,
    /// 离开时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<String>,
    /// AC 充电参数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,
    /// DC 充电参数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,
}

/// 能量转移模式
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EnergyTransferModeEnumType {
    DC,
    AC1Phase,
    AC3Phase,
}

/// AC 充电参数
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// 能量需求 (Wh)
    pub energy_amount: i32,
    /// EV 电量限值 (Wh)
    pub ev_min_current: i32,
    /// EV 最大电流 (A)
    pub ev_max_current: i32,
    /// EV 最大电压 (V)
    pub ev_max_voltage: i32,
}

/// DC 充电参数
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    /// EV 最大电流 (A)
    pub ev_max_current: i32,
    /// EV 最大电压 (V)
    pub ev_max_voltage: i32,
    /// 能量需求 (Wh, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<i32>,
    /// EV 最大电量 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<i32>,
    /// 状态电量 (SoC, 可选, %)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_of_charge: Option<i32>,
    /// EV 能量容量 (Wh, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<i32>,
    /// 全满 SoC (可选, %)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_soc: Option<i32>,
    /// 批量 SoC (可选, %)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_soc: Option<i32>,
}

impl NotifyEVChargingNeedsRequest {
    pub fn new(evse_id: i32, charging_needs: ChargingNeedsType) -> Self {
        Self {
            evse_id,
            charging_needs,
            max_schedule_tuples: None,
        }
    }
}

pub const ACTION: &str = "NotifyEVChargingNeeds";
