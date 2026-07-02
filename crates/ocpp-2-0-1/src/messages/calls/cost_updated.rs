//! CostUpdated Request (Core)
//! 推送事务实时费用

use serde::{Deserialize, Serialize};

/// CostUpdated 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// 运行中/最终费用
    pub total_cost: f64,
    /// 事务 ID (max 36 chars)
    pub transaction_id: String,
}

impl CostUpdatedRequest {
    pub fn new(total_cost: f64, transaction_id: impl Into<String>) -> Self {
        Self {
            total_cost,
            transaction_id: transaction_id.into(),
        }
    }
}

pub const ACTION: &str = "CostUpdated";