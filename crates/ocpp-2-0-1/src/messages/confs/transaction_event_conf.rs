//! TransactionEvent Confirmation (Functional Block E)

use serde::{Deserialize, Serialize};
use crate::common::{IdTokenInfoType, MessageContentType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
}

impl TransactionEventConfirmation {
    pub fn empty() -> Self {
        Self {
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            updated_personal_message: None,
        }
    }

    pub fn with_id_token_info(mut self, info: IdTokenInfoType) -> Self {
        self.id_token_info = Some(info);
        self
    }

    pub fn with_total_cost(mut self, cost: f64) -> Self {
        self.total_cost = Some(cost);
        self
    }
}

impl Default for TransactionEventConfirmation {
    fn default() -> Self {
        Self::empty()
    }
}