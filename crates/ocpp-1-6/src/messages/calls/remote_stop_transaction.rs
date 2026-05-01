//! RemoteStopTransaction 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::remote_stop_transaction_conf::RemoteStopTransactionConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: i32,
}

pub trait RemoteStopTransactionHandler: Send + Sync {
    fn handle(&self, req: RemoteStopTransactionRequest) -> RemoteStopTransactionConfirmation;
}

pub struct DefaultRemoteStopTransactionHandler;

impl Default for DefaultRemoteStopTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRemoteStopTransactionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl RemoteStopTransactionHandler for DefaultRemoteStopTransactionHandler {
    fn handle(&self, _req: RemoteStopTransactionRequest) -> RemoteStopTransactionConfirmation {
        RemoteStopTransactionConfirmation::accepted()
    }
}