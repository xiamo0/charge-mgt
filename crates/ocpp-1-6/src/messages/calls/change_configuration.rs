//! ChangeConfiguration 消息及处理器

use super::super::confs::change_configuration_conf::ChangeConfigurationConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

pub trait ChangeConfigurationHandler: Send + Sync {
    fn handle(&self, req: ChangeConfigurationRequest) -> ChangeConfigurationConfirmation;
}

pub struct DefaultChangeConfigurationHandler;

impl Default for DefaultChangeConfigurationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultChangeConfigurationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ChangeConfigurationHandler for DefaultChangeConfigurationHandler {
    fn handle(&self, _req: ChangeConfigurationRequest) -> ChangeConfigurationConfirmation {
        ChangeConfigurationConfirmation::accepted()
    }
}
