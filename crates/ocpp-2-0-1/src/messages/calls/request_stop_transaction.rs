//! RequestStopTransaction Request (Functional Block F)
//! 远程停止充电

use serde::{Deserialize, Serialize};

/// RequestStopTransaction 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    /// 事务 ID (max 36 chars)
    pub transaction_id: String,
}

impl RequestStopTransactionRequest {
    pub fn new(transaction_id: impl Into<String>) -> Self {
        Self {
            transaction_id: transaction_id.into(),
        }
    }
}

pub const ACTION: &str = "RequestStopTransaction";