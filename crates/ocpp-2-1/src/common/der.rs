//! OCPP 2.1 DER Control types (Functional Block R)

use serde::{Deserialize, Serialize};

/// DER 控制状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DERControlStatusEnumType {
    Accepted,
    Rejected,
    Unknown,
    NotSupported,
}

/// DER 控制类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DERControlEnumType {
    VoltWatt,
    VoltVar,
    FreqWatt,
    FixedPFAbsorb,
    FixedPFInject,
    FixedVar,
}

/// 电网事件故障枚举 (NotifyDERAlarm)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GridEventFaultEnumType {
    CurrentImbalance,
    LocalEmergency,
    LowInputPower,
    OverCurrent,
    OverFrequency,
    OverVoltage,
    PhaseRotation,
    RemoteEmergency,
    UnderFrequency,
    UnderVoltage,
    VoltageImbalance,
}

/// DER 曲线点类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DERPointType {
    pub x: f64,
    pub y: f64,
}

/// DER 曲线类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<DERPointType>>,
}

/// DER 曲线获取/报告类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveGetType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<DERPointType>>,
}

/// 投入服务类型 (DER)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_threshold: Option<f64>,
}

/// 固定无功类型 (DER)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedVarType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setpoint: Option<f64>,
}

/// 频率下垂类型 (DER)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

/// 梯度类型 (DER)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradientType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_rate: Option<f64>,
}

/// 最大放电限制类型 (DER)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
}
