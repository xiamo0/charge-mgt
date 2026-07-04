//! TransactionEvent Confirmation (Block E)
use crate::common::{IdTokenInfoType, MessageContentType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
}

impl Default for TransactionEventConfirmation {
    fn default() -> Self {
        Self {
            total_cost: None,
            charging_priority: None,
            updated_personal_message: None,
            id_token_info: None,
            offline: None,
        }
    }
}
