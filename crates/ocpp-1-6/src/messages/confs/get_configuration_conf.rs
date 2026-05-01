//! GetConfiguration 响应

use serde::{Deserialize, Serialize};
use crate::common::configuration::ConfigurationKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetConfigurationConfirmation {
    pub configuration_key: Vec<ConfigurationKey>,
    pub unknown_key: Vec<String>,
}

impl GetConfigurationConfirmation {
    pub fn empty() -> Self {
        Self {
            configuration_key: Vec::new(),
            unknown_key: Vec::new(),
        }
    }
}