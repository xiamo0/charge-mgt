//! ClearCache 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::clear_cache_conf::ClearCacheConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClearCacheRequest;

impl ClearCacheRequest {
    pub fn new() -> Self {
        Self
    }
}

pub trait ClearCacheHandler: Send + Sync {
    fn handle(&self, req: ClearCacheRequest) -> ClearCacheConfirmation;
}

pub struct DefaultClearCacheHandler;

impl Default for DefaultClearCacheHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultClearCacheHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ClearCacheHandler for DefaultClearCacheHandler {
    fn handle(&self, _req: ClearCacheRequest) -> ClearCacheConfirmation {
        ClearCacheConfirmation::accepted()
    }
}