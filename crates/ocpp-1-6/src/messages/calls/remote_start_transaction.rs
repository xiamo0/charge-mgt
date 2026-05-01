//! RemoteStartTransaction 消息及处理器

use super::super::confs::remote_start_transaction_conf::RemoteStartTransactionConfirmation;
use crate::common::configuration::ChargingProfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStartTransactionRequest {
    pub connector_id: Option<i32>,
    pub id_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfile>,
}

pub trait RemoteStartTransactionHandler: Send + Sync {
    fn handle(&self, req: RemoteStartTransactionRequest) -> RemoteStartTransactionConfirmation;
}

pub struct DefaultRemoteStartTransactionHandler;

impl Default for DefaultRemoteStartTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRemoteStartTransactionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl RemoteStartTransactionHandler for DefaultRemoteStartTransactionHandler {
    fn handle(&self, _req: RemoteStartTransactionRequest) -> RemoteStartTransactionConfirmation {
        RemoteStartTransactionConfirmation::accepted()
    }
}
