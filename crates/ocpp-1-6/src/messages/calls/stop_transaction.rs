//! StopTransaction 消息及处理器
//!
//! 定义停止交易请求、处理器 trait 以及默认实现。

use super::super::confs::stop_transaction_conf::StopTransactionConfirmation;
use crate::common::transaction::Reason;
use serde::{Deserialize, Serialize};

/// 停止交易请求，包含停止时读数、时间戳、事务 ID 以及可选的停止原因/交易数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StopTransactionRequest {
    /// 停止时电表读数
    pub meter_stop: i32,
    /// 停止时间（RFC3339 字符串）
    pub timestamp: String,
    /// 事务 ID
    pub transaction_id: i32,
    /// 停止原因（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    /// 可选的 idTag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>,
    /// 可选的交易数据（MeterValue 数组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<crate::common::meter_value::MeterValue>>,
}

/// StopTransaction 处理器接口
pub trait StopTransactionHandler: Send + Sync {
    fn handle(&self, req: StopTransactionRequest) -> StopTransactionConfirmation;
}

/// 默认的 StopTransaction 处理器（空实现，返回默认确认）
pub struct DefaultStopTransactionHandler;

impl Default for DefaultStopTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStopTransactionHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl StopTransactionHandler for DefaultStopTransactionHandler {
    fn handle(&self, _req: StopTransactionRequest) -> StopTransactionConfirmation {
        StopTransactionConfirmation::default()
    }
}
