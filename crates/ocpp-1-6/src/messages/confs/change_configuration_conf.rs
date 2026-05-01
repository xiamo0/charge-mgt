//! ChangeConfiguration 响应

use serde::{Deserialize, Serialize};
use crate::common::status::ConfigurationStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeConfigurationConfirmation {
    pub status: ConfigurationStatus,
}

impl ChangeConfigurationConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: ConfigurationStatus::Accepted,
        }
    }
}