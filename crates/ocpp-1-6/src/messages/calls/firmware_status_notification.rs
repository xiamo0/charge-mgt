//! FirmwareStatusNotification 消息及处理器（单向消息，无需回复）
//!
//! 用于上报固件更新状态（例如 Downloading、Installed 等），通常为单向通知无需回应。

use crate::common::status::FirmwareStatus;
use serde::{Deserialize, Serialize};

/// FirmwareStatusNotification 请求，包含固件状态与可选的 requestId
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirmwareStatusNotificationRequest {
    /// 固件当前状态
    pub status: FirmwareStatus,
    /// 可选的 request id，用于关联固件下载/安装任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

/// FirmwareStatusNotification 处理器接口（单向），不返回确认
pub trait FirmwareStatusNotificationHandler: Send + Sync {
    fn handle(&self, req: FirmwareStatusNotificationRequest);
}

/// 默认实现：空处理器
pub struct DefaultFirmwareStatusNotificationHandler;

impl Default for DefaultFirmwareStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultFirmwareStatusNotificationHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl FirmwareStatusNotificationHandler for DefaultFirmwareStatusNotificationHandler {
    /// 默认不做任何处理
    fn handle(&self, _req: FirmwareStatusNotificationRequest) {}
}
