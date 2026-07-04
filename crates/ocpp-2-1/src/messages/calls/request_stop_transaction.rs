//! RequestStopTransaction Request (Block F)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    pub transaction_id: String,
}

pub const ACTION: &str = "RequestStopTransaction";
