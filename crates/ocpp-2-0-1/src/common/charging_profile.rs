//! Charging Profile Types (Functional Block H)

use serde::{Deserialize, Serialize};

/// 充电档案用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfilePurposeEnumType {
    /// 充电站最大档案
    ChargingStationMaxProfile,
    /// 充电站外部约束
    ChargingStationExternalConstraints,
    /// 事务默认档案
    TxDefaultProfile,
    /// 事务档案
    TxProfile,
}

/// 充电档案类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileKindEnumType {
    /// 绝对
    Absolute,
    /// 相对
    Relative,
    /// 周期性
    Recurring,
}

/// 重复性类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RecurrencyKindEnumType {
    /// 每日
    Daily,
    /// 每周
    Weekly,
}

/// 充电速率单位枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingRateUnitEnumType {
    /// 瓦特
    W,
    /// 安培
    A,
}

/// 充电调度周期类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    /// 起始周期 (秒)
    pub start_period: i32,
    /// 限制值
    pub limit: f64,
    /// 使用相数 (默认 3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
    /// 使用相位 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,
}

impl ChargingSchedulePeriodType {
    /// 创建新的调度周期
    pub fn new(start_period: i32, limit: f64) -> Self {
        Self {
            start_period,
            limit,
            number_phases: None,
            phase_to_use: None,
        }
    }

    /// 设置相数
    pub fn with_number_phases(mut self, phases: i32) -> Self {
        self.number_phases = Some(phases);
        self
    }
}

/// 销售电价类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    /// 电价ID
    pub id: i32,
    /// 电价描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<String>,
    /// 电价级别数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,
    /// 电价条目
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

/// 销售电价条目类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    /// 相对时间间隔
    pub relative_time_interval: RelativeTimeIntervalType,
    /// 电价级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<i32>,
    /// 消费成本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
}

/// 相对时间间隔类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    /// 起始时间 (秒)
    pub start: i32,
    /// 持续时间 (秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

/// 消费成本类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    /// 起始值
    pub start_value: f64,
    /// 成本
    pub cost: Vec<CostType>,
}

/// 成本类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostType {
    /// 成本类型
    pub cost_kind: CostKindEnumType,
    /// 金额
    pub amount: i32,
    /// 金额倍数 (默认 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_multiplier: Option<i32>,
}

/// 成本类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CostKindEnumType {
    /// 碳成本
    Carbon,
    /// 可再生能源百分比
    RenewableEnergy,
    /// 电价
    EnergyPrice,
    /// 二氧化碳排放
    CO2,
}

/// 充电调度类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    /// 调度ID
    pub id: i32,
    /// 持续时间 (秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 起始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<String>,
    /// 充电速率单位
    pub charging_rate_unit: ChargingRateUnitEnumType,
    /// 调度周期列表
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    /// 最小充电速率 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
    /// 销售电价 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTariffType>,
}

impl ChargingScheduleType {
    /// 创建新的调度
    pub fn new(
        id: i32,
        unit: ChargingRateUnitEnumType,
        periods: Vec<ChargingSchedulePeriodType>,
    ) -> Self {
        Self {
            id,
            duration: None,
            start_schedule: None,
            charging_rate_unit: unit,
            charging_schedule_period: periods,
            min_charging_rate: None,
            sales_tariff: None,
        }
    }

    /// 设置持续时间
    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// 设置最小充电速率
    pub fn with_min_charging_rate(mut self, rate: f64) -> Self {
        self.min_charging_rate = Some(rate);
        self
    }
}

/// 充电档案类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    /// 档案ID
    pub id: i32,
    /// 栈级别
    pub stack_level: i32,
    /// 档案用途
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    /// 档案类型
    pub charging_profile_kind: ChargingProfileKindEnumType,
    /// 重复性类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    /// 有效起始时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// 有效结束时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    /// 事务ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 调度列表
    pub charging_schedule: Vec<ChargingScheduleType>,
}

impl ChargingProfileType {
    /// 创建新的充电档案
    pub fn new(
        id: i32,
        stack_level: i32,
        purpose: ChargingProfilePurposeEnumType,
        kind: ChargingProfileKindEnumType,
        schedules: Vec<ChargingScheduleType>,
    ) -> Self {
        Self {
            id,
            stack_level,
            charging_profile_purpose: purpose,
            charging_profile_kind: kind,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            transaction_id: None,
            charging_schedule: schedules,
        }
    }

    /// 设置有效时间
    pub fn with_valid_period(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.valid_from = Some(from.into());
        self.valid_to = Some(to.into());
        self
    }

    /// 设置事务ID
    pub fn with_transaction_id(mut self, transaction_id: impl Into<String>) -> Self {
        self.transaction_id = Some(transaction_id.into());
        self
    }

    /// 设置重复性
    pub fn with_recurrency_kind(mut self, recurrency_kind: RecurrencyKindEnumType) -> Self {
        self.recurrency_kind = Some(recurrency_kind);
        self
    }
}

/// 设置充电档案状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChargingProfileStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
}

/// 清除充电档案状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearChargingProfileStatusEnumType {
    ///已接受
    Accepted,
    /// 未知
    Unknown,
}

/// 获取复合调度状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GenericDeviceModelStatusEnumType {
    /// 已接受
    Accepted,
    /// 已拒绝
    Rejected,
    /// 不支持
    NotSupported,
    /// 空结果集
    EmptyResultSet,
}

/// CompositeScheduleType - GetCompositeSchedule 响应的复合调度结构
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeScheduleType {
    pub evse_id: i32,
    pub duration: i32,
    pub schedule_start: String,
    pub charging_rate_unit: ChargingRateUnitEnumType,
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charging_profile_purpose_enum() {
        let variants = [
            ChargingProfilePurposeEnumType::ChargingStationMaxProfile,
            ChargingProfilePurposeEnumType::TxDefaultProfile,
            ChargingProfilePurposeEnumType::TxProfile,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargingProfilePurposeEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_charging_profile_kind_enum() {
        let variants = [
            ChargingProfileKindEnumType::Absolute,
            ChargingProfileKindEnumType::Relative,
            ChargingProfileKindEnumType::Recurring,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ChargingProfileKindEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_charging_schedule_period() {
        let period = ChargingSchedulePeriodType::new(0, 11000.0)
            .with_number_phases(3);
        let json = serde_json::to_string(&period).unwrap();
        let de: ChargingSchedulePeriodType = serde_json::from_str(&json).unwrap();
        assert_eq!(period, de);
    }

    #[test]
    fn test_charging_schedule() {
        let schedule = ChargingScheduleType::new(
            1,
            ChargingRateUnitEnumType::W,
            vec![ChargingSchedulePeriodType::new(0, 7000.0)],
        );
        let json = serde_json::to_string(&schedule).unwrap();
        let de: ChargingScheduleType = serde_json::from_str(&json).unwrap();
        assert_eq!(schedule, de);
    }

    #[test]
    fn test_charging_profile() {
        let profile = ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::TxProfile,
            ChargingProfileKindEnumType::Relative,
            vec![ChargingScheduleType::new(
                1,
                ChargingRateUnitEnumType::W,
                vec![ChargingSchedulePeriodType::new(0, 11000.0)],
            )],
        );
        let json = serde_json::to_string(&profile).unwrap();
        let de: ChargingProfileType = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, de);
    }
}