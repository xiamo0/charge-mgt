//! ChangeAvailability 消息及处理器
//!
//! 更改连接器可用性（例如将连接器置为可用或暂停）请求与默认处理器。

use super::super::confs::change_availability_conf::ChangeAvailabilityConfirmation;
use crate::common::status::AvailabilityType;
use serde::{Deserialize, Serialize};

/// ChangeAvailability 请求，包含要操作的连接器 ID 和目标可用性类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeAvailabilityRequest {
    /// 连接器编号
    pub connector_id: i32,
    /// 目标可用性类型（Operative / Suspended / Inoperative）
    #[serde(rename = "type")]
    pub availability_type: AvailabilityType,
}

/// ChangeAvailability 处理器接口
pub trait ChangeAvailabilityHandler: Send + Sync {
    fn handle(&self, req: ChangeAvailabilityRequest) -> ChangeAvailabilityConfirmation;
}

/// 默认实现：直接接受更改请求
pub struct DefaultChangeAvailabilityHandler;

impl Default for DefaultChangeAvailabilityHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultChangeAvailabilityHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl ChangeAvailabilityHandler for DefaultChangeAvailabilityHandler {
    fn handle(&self, _req: ChangeAvailabilityRequest) -> ChangeAvailabilityConfirmation {
        ChangeAvailabilityConfirmation::accepted()
    }
}
