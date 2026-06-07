//! ReserveNow 消息及处理器
//!
//! 预订充电（ReserveNow）的请求与处理器，默认实现直接接受预订。

use super::super::confs::reserve_now_conf::ReserveNowConfirmation;
use serde::{Deserialize, Serialize};

/// ReserveNow 请求
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReserveNowRequest {
    /// 连接器编号
    pub connector_id: i32,
    /// 预订失效时间（RFC3339 字符串）
    pub expiry_date: String,
    /// idTag（预订者）
    pub id_tag: String,
    /// 预订 ID
    pub reservation_id: i32,
    /// 可选的父 idTag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
}

/// ReserveNow 处理器接口
pub trait ReserveNowHandler: Send + Sync {
    fn handle(&self, req: ReserveNowRequest) -> ReserveNowConfirmation;
}

/// 默认实现：接受预订
pub struct DefaultReserveNowHandler;

impl Default for DefaultReserveNowHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultReserveNowHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl ReserveNowHandler for DefaultReserveNowHandler {
    fn handle(&self, _req: ReserveNowRequest) -> ReserveNowConfirmation {
        ReserveNowConfirmation::accepted()
    }
}
