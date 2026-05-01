//! StopTransaction 消息及处理器

use super::super::confs::stop_transaction_conf::StopTransactionConfirmation;
use crate::common::transaction::Reason;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopTransactionRequest {
    pub meter_stop: i32,
    pub timestamp: String,
    pub transaction_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<crate::common::meter_value::MeterValue>>,
}

pub trait StopTransactionHandler: Send + Sync {
    fn handle(&self, req: StopTransactionRequest) -> StopTransactionConfirmation;
}

pub struct DefaultStopTransactionHandler;

impl Default for DefaultStopTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStopTransactionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl StopTransactionHandler for DefaultStopTransactionHandler {
    fn handle(&self, _req: StopTransactionRequest) -> StopTransactionConfirmation {
        StopTransactionConfirmation::default()
    }
}
