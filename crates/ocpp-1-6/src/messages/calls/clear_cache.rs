//! ClearCache 消息及处理器
//!
//! 定义清除本地缓存请求类型与处理器接口，默认实现直接返回 accepted。

use super::super::confs::clear_cache_conf::ClearCacheConfirmation;
use serde::{Deserialize, Serialize};

/// ClearCache 请求（空结构体）
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ClearCacheRequest;

impl ClearCacheRequest {
    /// 构造一个新的 ClearCacheRequest
    pub fn new() -> Self {
        Self
    }
}

/// ClearCache 处理器接口
pub trait ClearCacheHandler: Send + Sync {
    fn handle(&self, req: ClearCacheRequest) -> ClearCacheConfirmation;
}

/// 默认 ClearCache 处理器实现
pub struct DefaultClearCacheHandler;

impl Default for DefaultClearCacheHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultClearCacheHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl ClearCacheHandler for DefaultClearCacheHandler {
    fn handle(&self, _req: ClearCacheRequest) -> ClearCacheConfirmation {
        ClearCacheConfirmation::accepted()
    }
}
