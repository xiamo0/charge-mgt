//! SendLocalList 消息及处理器
//!
//! 下发本地授权列表（Local Authorization List）到充电点的请求与处理器。

use super::super::confs::send_local_list_conf::SendLocalListConfirmation;
use crate::common::authorization_list::AuthorizationList;
use crate::common::status::UpdateType;
use serde::{Deserialize, Serialize};

/// SendLocalList 请求结构，包含目标列表版本、可选数据与更新类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendLocalListRequest {
    /// 列表版本（字符串形式）
    pub list_version: String,
    /// 可选的本地授权列表数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationList>>,
    /// 更新类型（Full / Differential）
    #[serde(rename = "updateType")]
    pub update_type: UpdateType,
}

/// SendLocalList 的处理器接口
pub trait SendLocalListHandler: Send + Sync {
    fn handle(&self, req: SendLocalListRequest) -> SendLocalListConfirmation;
}

/// 默认实现：接受下发的列表
pub struct DefaultSendLocalListHandler;

impl Default for DefaultSendLocalListHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSendLocalListHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl SendLocalListHandler for DefaultSendLocalListHandler {
    fn handle(&self, _req: SendLocalListRequest) -> SendLocalListConfirmation {
        SendLocalListConfirmation::accepted()
    }
}
