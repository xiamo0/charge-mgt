//! RemoteStopTransaction 消息及处理器
//!
//! 远程停止事务请求定义及默认处理器实现。

use super::super::confs::remote_stop_transaction_conf::RemoteStopTransactionConfirmation;
use serde::{Deserialize, Serialize};

/// RemoteStopTransaction 请求，包含要停止的事务 ID
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStopTransactionRequest {
    /// 要停止的 transactionId
    pub transaction_id: i32,
}

/// 处��� RemoteStopTransaction 的 trait
pub trait RemoteStopTransactionHandler: Send + Sync {
    fn handle(&self, req: RemoteStopTransactionRequest) -> RemoteStopTransactionConfirmation;
}

/// 默认实现：返回 accepted
pub struct DefaultRemoteStopTransactionHandler;

impl Default for DefaultRemoteStopTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRemoteStopTransactionHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl RemoteStopTransactionHandler for DefaultRemoteStopTransactionHandler {
    fn handle(&self, _req: RemoteStopTransactionRequest) -> RemoteStopTransactionConfirmation {
        RemoteStopTransactionConfirmation::accepted()
    }
}
