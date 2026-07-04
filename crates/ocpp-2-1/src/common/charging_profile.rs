//! Charging Profile Types (Functional Block K — SmartCharging)

use serde::{Deserialize, Serialize};

/// 充电档案用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfilePurposeEnumType {
    ChargingStationMaxProfile,
    ChargingStationExternalConstraints,
    TxDefaultProfile,
    TxProfile,
}

/// 充电档案类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileKindEnumType {
    Absolute,
    Relative,
    Recurring,
}

/// 重复性类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RecurrencyKindEnumType {
    Daily,
    Weekly,
}

/// 充电速率单位枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingRateUnitEnumType {
    W,
    A,
}

/// 充电调度周期类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    pub start_period: i32,
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,
}

impl ChargingSchedulePeriodType {
    pub fn new(start_period: i32, limit: f64) -> Self {
        Self {
            start_period,
            limit,
            number_phases: None,
            phase_to_use: None,
        }
    }
}

/// 销售电价类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

/// 销售电价条目类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
}

/// 相对时间间隔类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    pub start: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

/// 消费成本类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    pub start_value: f64,
    pub cost: Vec<CostType>,
}

/// 成本类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostType {
    pub cost_kind: CostKindEnumType,
    pub amount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i32>,
}

/// 成本类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CostKindEnumType {
    Carbon,
    RenewableEnergy,
    EnergyPrice,
    CO2,
}

/// 充电调度类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<String>,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTariffType>,
}

/// 充电档案类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    pub id: i32,
    pub stack_level: i32,
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    pub charging_profile_kind: ChargingProfileKindEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub charging_schedule: Vec<ChargingScheduleType>,
}

/// 设置充电档案状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileStatusEnumType {
    Accepted,
    Rejected,
}

/// 清除充电档案状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearChargingProfileStatusEnumType {
    Accepted,
    Unknown,
}

/// 获取复合调度状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GenericDeviceModelStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
    EmptyResultSet,
}

/// 复合调度类型 - GetCompositeSchedule 响应
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeScheduleType {
    pub evse_id: i32,
    pub duration: i32,
    pub schedule_start: String,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

/// 充电曲线过滤条件 (用于 GetChargingProfiles)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_limit_source: Option<Vec<ChargingLimitSourceEnumType>>,
}

/// 充电限制来源枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingLimitSourceEnumType {
    EMSO,
    Other,
    SO,
    CSO,
}

/// 清除充电曲线过滤条件 (用于 ClearChargingProfile)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

/// 充电限制类型 (用于 NotifyChargingLimit)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,
}

/// 充电需求类型 (用于 NotifyEVChargingNeeds — ISO 15118)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    pub requested_energy_transfer: ChargingRateUnitEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_request: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_current: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_energy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_min_current: Option<f64>,
}

/// 充电调度更新类型 (2.1 — PullDynamicScheduleUpdate / UpdateDynamicSchedule)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleUpdateType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discharge_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint_reactive: Option<f64>,
}

/// 优先级充电状态枚举 (2.1 — UsePriorityCharging)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PriorityChargingStatusEnumType {
    Accepted,
    Rejected,
    NoProfile,
}
