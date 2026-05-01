//! ClearCache 响应

use crate::common::status::ClearCacheStatus;
use serde::{Deserialize, Serialize};

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
