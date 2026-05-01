//! ClearCache 响应

use serde::{Deserialize, Serialize};
use crate::common::status::ClearCacheStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearCacheConfirmation {
    pub status: ClearCacheStatus,
}

impl ClearCacheConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: ClearCacheStatus::Accepted,
        }
    }
}