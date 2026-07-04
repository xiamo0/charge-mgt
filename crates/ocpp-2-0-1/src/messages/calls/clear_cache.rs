//! ClearCache Request (Core)
//! 清除本地授权缓存

use serde::{Deserialize, Serialize};

/// ClearCache 请求 (空 payload)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearCacheRequest {}

impl ClearCacheRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ClearCacheRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "ClearCache";
