//! CostUpdated Request (Block I)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    pub total_cost: f64,
    pub transaction_id: String,
}

pub const ACTION: &str = "CostUpdated";
