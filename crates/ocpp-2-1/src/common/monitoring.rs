//! OCPP 2.0.1 / 2.1 Monitoring types (Functional Block N — Diagnostics)

use crate::common::{ComponentType, StatusInfoType, VariableType};
use serde::{Deserialize, Serialize};

/// 监控器类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    Delta,
    Periodic,
    PeriodicClockAligned,
}

/// 设置监控数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    pub value: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<MonitorEnumType>,
    pub severity: i32,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
}

/// 设置监控状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    DuplicateValue,
}

/// 设置监控结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    pub status: SetMonitoringStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<MonitorEnumType>,
    pub severity: i32,
    pub component: ComponentType,
    pub variable: VariableType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// 清除监控状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClearMonitoringStatusEnumType {
    Accepted,
    Rejected,
    NotFound,
}

/// 清除监控结果类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    pub status: ClearMonitoringStatusEnumType,
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

/// 监控标准枚举 (GetMonitoringReport)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringCriterionEnumType {
    ThresholdMonitoring,
    DeltaMonitoring,
    PeriodicMonitoring,
}

/// 变量监控项类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    pub id: i32,
    pub transaction: bool,
    pub value: String,
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,
    pub severity: i32,
}

/// 监控数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_monitoring: Vec<VariableMonitoringType>,
}

/// 监控基础枚举 (SetMonitoringBase)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}
