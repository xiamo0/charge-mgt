//! StartTransaction 消息及处理器
//!
//! 启动交易的请求与处理器接口，包含必要的连接器编号、idTag 与初始电表读数。

use super::super::confs::start_transaction_conf::StartTransactionConfirmation;
use serde::{Deserialize, Serialize};

/// StartTransaction 请求结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartTransactionRequest {
    /// 连接器编号
    pub connector_id: i32,
    /// idTag，用于授权启动充电
    pub id_tag: String,
    /// 启动时的电表读数
    pub meter_start: i32,
    /// 可选的 reservationId（若使用预约功能）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    /// 时间戳（RFC3339 字符串）
    pub timestamp: String,
}

/// StartTransaction 处理器接口
pub trait StartTransactionHandler: Send + Sync {
    fn handle(&self, req: StartTransactionRequest) -> StartTransactionConfirmation;
}

/// 默认实现：返回一个示例 transaction id
pub struct DefaultStartTransactionHandler;

impl Default for DefaultStartTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStartTransactionHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl StartTransactionHandler for DefaultStartTransactionHandler {
    fn handle(&self, _req: StartTransactionRequest) -> StartTransactionConfirmation {
        // 返回一个示例 transaction id（生产中应由后端生成真实 ID）
        StartTransactionConfirmation::new(1)
    }
}
