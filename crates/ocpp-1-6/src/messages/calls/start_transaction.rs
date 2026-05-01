//! StartTransaction 消息及处理器

use super::super::confs::start_transaction_conf::StartTransactionConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartTransactionRequest {
    pub connector_id: i32,
    pub id_tag: String,
    pub meter_start: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    pub timestamp: String,
}

pub trait StartTransactionHandler: Send + Sync {
    fn handle(&self, req: StartTransactionRequest) -> StartTransactionConfirmation;
}

pub struct DefaultStartTransactionHandler;

impl Default for DefaultStartTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStartTransactionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl StartTransactionHandler for DefaultStartTransactionHandler {
    fn handle(&self, _req: StartTransactionRequest) -> StartTransactionConfirmation {
        StartTransactionConfirmation::new(1)
    }
}
