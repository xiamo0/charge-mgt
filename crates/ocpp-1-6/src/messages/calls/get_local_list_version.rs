//! GetLocalListVersion 消息及处理器
//!
//! 查询本地授权列表版本号的请求与处理器。

use super::super::confs::get_local_list_version_conf::GetLocalListVersionConfirmation;
use serde::{Deserialize, Serialize};

/// GetLocalListVersion 请求（空结构体）
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetLocalListVersionRequest;

impl GetLocalListVersionRequest {
    /// 创建请求实例
    pub fn new() -> Self {
        Self
    }
}

/// 处理 GetLocalListVersion 请求的 trait
pub trait GetLocalListVersionHandler: Send + Sync {
    fn handle(&self, req: GetLocalListVersionRequest) -> GetLocalListVersionConfirmation;
}

/// 默认实现：返回版本号 0
pub struct DefaultGetLocalListVersionHandler;

impl Default for DefaultGetLocalListVersionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetLocalListVersionHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl GetLocalListVersionHandler for DefaultGetLocalListVersionHandler {
    fn handle(&self, _req: GetLocalListVersionRequest) -> GetLocalListVersionConfirmation {
        GetLocalListVersionConfirmation::new(0)
    }
}
