//! SetMonitoringLevel Request (Functional Block D)
//! 设置监控严重度阈值

use serde::{Deserialize, Serialize};

/// SetMonitoringLevel 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    /// 严重度阈值 (0-9, 只上报严重度 >= 此阈值的监控事件)
    pub severity: i32,
}

impl SetMonitoringLevelRequest {
    pub fn new(severity: i32) -> Self {
        Self { severity }
    }
}

pub const ACTION: &str = "SetMonitoringLevel";