//! ClearVariableMonitoring Request (Functional Block D)
//! 清除变量监控

use serde::{Deserialize, Serialize};

/// ClearVariableMonitoring 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    /// 要清除的监控 ID 列表
    pub id: Vec<i32>,
}

impl ClearVariableMonitoringRequest {
    pub fn new(ids: Vec<i32>) -> Self {
        Self { id: ids }
    }
}

pub const ACTION: &str = "ClearVariableMonitoring";
