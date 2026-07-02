//! GetTransactionStatus Confirmation

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusConfirmation {
    pub messages_in_queue: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,
}

impl GetTransactionStatusConfirmation {
    pub fn new(messages_in_queue: bool) -> Self {
        Self {
            messages_in_queue,
            ongoing_indicator: None,
        }
    }

    pub fn with_ongoing(mut self, ongoing: bool) -> Self {
        self.ongoing_indicator = Some(ongoing);
        self
    }
}