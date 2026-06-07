//! UnlockConnector 消息及处理器
//!
//! 解锁连接器请求和默认处理器，实现对连接器解锁的控制。

use super::super::confs::unlock_connector_conf::UnlockConnectorConfirmation;
use serde::{Deserialize, Serialize};

/// UnlockConnector 请求，包含要解锁的连接器编号
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnlockConnectorRequest {
    /// 要解锁的连接器 ID
    pub connector_id: i32,
}

/// UnlockConnector 处理器接口
pub trait UnlockConnectorHandler: Send + Sync {
    fn handle(&self, req: UnlockConnectorRequest) -> UnlockConnectorConfirmation;
}

/// 默认实现：返回 unlocked
pub struct DefaultUnlockConnectorHandler;

impl Default for DefaultUnlockConnectorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultUnlockConnectorHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl UnlockConnectorHandler for DefaultUnlockConnectorHandler {
    fn handle(&self, _req: UnlockConnectorRequest) -> UnlockConnectorConfirmation {
        UnlockConnectorConfirmation::unlocked()
    }
}
