//! SetVariableMonitoring Request (Functional Block D)
//! 设置变量监控

use serde::{Deserialize, Serialize};
use crate::common::{ComponentType, VariableType};

/// 监控标准枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitorEnumType {
    UpperThreshold,
    LowerThreshold,
    Delta,
    Periodic,
    PeriodicClockAligned,
}

/// 监控数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    /// 监控值
    pub value: f64,
    /// 监控类型
    #[serde(rename = "type")]
    pub monitor_type: MonitorEnumType,
    /// 严重度 (0-9)
    pub severity: i32,
    /// 组件
    pub component: ComponentType,
    /// 变量
    pub variable: VariableType,
    /// 事务标志 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
    /// ID (可选, 修改已有监控时使用)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// SetVariableMonitoring 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    /// 监控数据列表
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

impl SetVariableMonitoringRequest {
    pub fn new(set_monitoring_data: Vec<SetMonitoringDataType>) -> Self {
        Self { set_monitoring_data }
    }
}

pub const ACTION: &str = "SetVariableMonitoring";