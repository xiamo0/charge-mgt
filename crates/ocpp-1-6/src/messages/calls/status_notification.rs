//! StatusNotification 消息及处理器
//!
//! 充电点状态上报（StatusNotification），包含连接器状态、错误码、可选厂商错误码与时间戳等。

use super::super::confs::status_notification_conf::StatusNotificationConfirmation;
use crate::common::status::{ChargePointErrorCode, ChargePointStatus};
use serde::{Deserialize, Serialize};

/// StatusNotification 请求，携带连接器状态信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusNotificationRequest {
    /// 连接器编号
    pub connector_id: i32,
    /// 充电点错误码（枚举）
    pub error_code: ChargePointErrorCode,
    /// 可��的额外信息文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// 当前连接器状态（枚举）
    pub status: ChargePointStatus,
    /// 时间戳（RFC3339）
    pub timestamp: String,
    /// 可选厂商 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    /// 可选厂商自定义错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_error_code: Option<String>,
}

/// StatusNotification 处理器接口
pub trait StatusNotificationHandler: Send + Sync {
    fn handle(&self, req: StatusNotificationRequest) -> StatusNotificationConfirmation;
}

/// 默认实现：空处理器，返回空确认
pub struct DefaultStatusNotificationHandler;

impl Default for DefaultStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultStatusNotificationHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl StatusNotificationHandler for DefaultStatusNotificationHandler {
    fn handle(&self, _req: StatusNotificationRequest) -> StatusNotificationConfirmation {
        StatusNotificationConfirmation
    }
}
