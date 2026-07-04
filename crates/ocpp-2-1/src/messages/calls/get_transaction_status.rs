//! GetTransactionStatus Request (Block E)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

pub const ACTION: &str = "GetTransactionStatus";
