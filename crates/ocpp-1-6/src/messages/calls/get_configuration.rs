//! GetConfiguration 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::get_configuration_conf::GetConfigurationConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetConfigurationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

pub trait GetConfigurationHandler: Send + Sync {
    fn handle(&self, req: GetConfigurationRequest) -> GetConfigurationConfirmation;
}

pub struct DefaultGetConfigurationHandler;

impl Default for DefaultGetConfigurationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetConfigurationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl GetConfigurationHandler for DefaultGetConfigurationHandler {
    fn handle(&self, _req: GetConfigurationRequest) -> GetConfigurationConfirmation {
        GetConfigurationConfirmation::empty()
    }
}