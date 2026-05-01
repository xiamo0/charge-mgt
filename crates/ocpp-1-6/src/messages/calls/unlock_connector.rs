//! UnlockConnector 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::unlock_connector_conf::UnlockConnectorConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnlockConnectorRequest {
    pub connector_id: i32,
}

pub trait UnlockConnectorHandler: Send + Sync {
    fn handle(&self, req: UnlockConnectorRequest) -> UnlockConnectorConfirmation;
}

pub struct DefaultUnlockConnectorHandler;

impl Default for DefaultUnlockConnectorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultUnlockConnectorHandler {
    pub fn new() -> Self {
        Self
    }
}

impl UnlockConnectorHandler for DefaultUnlockConnectorHandler {
    fn handle(&self, _req: UnlockConnectorRequest) -> UnlockConnectorConfirmation {
        UnlockConnectorConfirmation::unlocked()
    }
}