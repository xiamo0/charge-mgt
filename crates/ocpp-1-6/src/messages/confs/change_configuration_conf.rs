//! ChangeConfiguration 响应

use crate::common::status::ConfigurationStatus;
use serde::{Deserialize, Serialize};

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
