//! GetTransactionStatus Confirmation (Block E)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,
    pub messages_in_queue: bool,
}
