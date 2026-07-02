//! GetTransactionStatus Request (Core)
//! 查询事务状态

use serde::{Deserialize, Serialize};

/// GetTransactionStatus 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    /// 事务 ID (可选, max 36 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl GetTransactionStatusRequest {
    pub fn new() -> Self {
        Self {
            transaction_id: None,
        }
    }

    pub fn for_transaction(transaction_id: impl Into<String>) -> Self {
        Self {
            transaction_id: Some(transaction_id.into()),
        }
    }
}

impl Default for GetTransactionStatusRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "GetTransactionStatus";