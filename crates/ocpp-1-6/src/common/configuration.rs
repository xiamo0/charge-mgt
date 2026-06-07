//! 配置类型
//!
//! 包含 OCPP 配置相关的数据结构，例如配置键值、授权数据、本地授权列表以及充电曲线/配置文件等。

use serde::{Deserialize, Serialize};

/// 表示一个配置项（key/value/readonly）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationKey {
    /// 配置键名
    pub key: String,
    /// 配置值
    pub value: String,
    /// 是否只读
    #[serde(rename = "readonly")]
    pub readonly: bool,
}

/// 简单的键值对结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValue {
    /// 键名
    pub key: String,
    /// 值
    pub value: String,
}

/// 授权数据项（用于本地授权列表项）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationData {
    /// idTag 字段
    #[serde(rename = "idTag")]
    pub id_tag: String,
    /// 可选的 idTag 信息
    #[serde(rename = "idTagInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<crate::common::id_tag::IdTagInfo>,
}

/// 本地授权列表结构，包含可选的授权数据数组和版本号
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalAuthorizeList {
    /// 本地授权列表（可选）
    #[serde(rename = "localAuthorizationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    /// 列表版本号
    #[serde(rename = "listVersion")]
    pub list_version: i32,
}

/// 充电调度（ChargingSchedule），描述充电周期和限制
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingSchedule {
    /// 调度持续时长（秒），可选
    pub duration: Option<i32>,
    /// 调度开始时段（秒），可选
    #[serde(rename = "startPeriod")]
    pub start_period: Option<i32>,
    /// 充电速率单位（如 W 或 A）
    #[serde(rename = "chargingRateUnit")]
    pub charging_rate_unit: String,
    /// 本充电调度包含的多个时间段
    #[serde(rename = "chargingSchedulePeriod")]
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    /// 最小充电速率（可选）
    #[serde(rename = "minChargingRate")]
    pub min_charging_rate: Option<f64>,
}

/// 单个充电调度时间段
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingSchedulePeriod {
    /// 本段的起始秒数
    #[serde(rename = "startPeriod")]
    pub start_period: i32,
    /// 限制值（功率或电流）
    pub limit: f64,
    /// 相数（可选）
    #[serde(rename = "numberPhases")]
    pub number_phases: Option<i32>,
}

/// 充电档案（ChargingProfile）描述一套完整的充电策略
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargingProfile {
    #[serde(rename = "chargingProfileId")]
    pub charging_profile_id: i32,
    #[serde(rename = "stackLevel")]
    pub stack_level: i32,
    #[serde(rename = "chargingProfilePurpose")]
    pub charging_profile_purpose: String,
    #[serde(rename = "chargingProfileKind")]
    pub charging_profile_kind: String,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = " ChargingSchedule")]
    pub charging_schedule: Option<ChargingSchedule>,
    #[serde(rename = "recurrencyKind")]
    pub recurrency_kind: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<i32>,
}
