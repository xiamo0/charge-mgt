//! SetMonitoringBase Request (Functional Block D)
//! 激活出厂默认监控配置

use serde::{Deserialize, Serialize};

/// 监控基础枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringBaseEnumType {
    All,
    FactoryDefault,
    HardWiredOnly,
}

/// SetMonitoringBase 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringBaseRequest {
    /// 监控基础类型
    pub monitoring_base: MonitoringBaseEnumType,
}

impl SetMonitoringBaseRequest {
    pub fn new(monitoring_base: MonitoringBaseEnumType) -> Self {
        Self { monitoring_base }
    }
}

pub const ACTION: &str = "SetMonitoringBase";