//! DiagnosticsStatusNotification 消息及处理器（单向消息，无需回复）
//!
//! 用于上报诊断状态（例如 Idle / Downloading / Installed 等），该消息为单向通知，通常不需要确认。

use crate::common::status::DiagnosticsStatus;
use serde::{Deserialize, Serialize};

/// DiagnosticsStatusNotification 请求，包含当前诊断状态与可选的 requestId
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticsStatusNotificationRequest {
    /// 当前诊断状态
    pub status: DiagnosticsStatus,
    /// 可选的 request id，用于关联上传/下载任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

/// DiagnosticsStatusNotification 的处理器接口（单向），不返回确认
pub trait DiagnosticsStatusNotificationHandler: Send + Sync {
    fn handle(&self, req: DiagnosticsStatusNotificationRequest);
}

/// 默认实现：空处理器
pub struct DefaultDiagnosticsStatusNotificationHandler;

impl Default for DefaultDiagnosticsStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultDiagnosticsStatusNotificationHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl DiagnosticsStatusNotificationHandler for DefaultDiagnosticsStatusNotificationHandler {
    /// 默认不做任何处理
    fn handle(&self, _req: DiagnosticsStatusNotificationRequest) {}
}
