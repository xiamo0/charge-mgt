//! RemoteStartTransaction 消息及处理器
//!
//! 远程启动事务请求，允许后台向充电桩发起远程启动操作，携带可选的充电档案

use super::super::confs::remote_start_transaction_conf::RemoteStartTransactionConfirmation;
use crate::common::configuration::ChargingProfile;
use serde::{Deserialize, Serialize};

/// RemoteStartTransaction 请求
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteStartTransactionRequest {
    /// 可选的连接器编号（若无则由充电点选择）
    pub connector_id: Option<i32>,
    /// 授权 idTag，用于开启充电
    pub id_tag: String,
    /// 可选的 ChargingProfile，用于指定充电参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfile>,
}

/// 处理 RemoteStartTransaction 的 trait
pub trait RemoteStartTransactionHandler: Send + Sync {
    fn handle(&self, req: RemoteStartTransactionRequest) -> RemoteStartTransactionConfirmation;
}

/// 默认实现：始终返回 accepted
pub struct DefaultRemoteStartTransactionHandler;

impl Default for DefaultRemoteStartTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRemoteStartTransactionHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl RemoteStartTransactionHandler for DefaultRemoteStartTransactionHandler {
    fn handle(&self, _req: RemoteStartTransactionRequest) -> RemoteStartTransactionConfirmation {
        RemoteStartTransactionConfirmation::accepted()
    }
}
