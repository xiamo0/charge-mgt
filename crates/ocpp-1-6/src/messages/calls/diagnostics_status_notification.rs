//! DiagnosticsStatusNotification 消息及处理器（单向消息，无需回复）

use serde::{Deserialize, Serialize};
use crate::common::status::DiagnosticsStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticsStatusNotificationRequest {
    pub status: DiagnosticsStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

pub trait DiagnosticsStatusNotificationHandler: Send + Sync {
    fn handle(&self, req: DiagnosticsStatusNotificationRequest);
}

pub struct DefaultDiagnosticsStatusNotificationHandler;

impl Default for DefaultDiagnosticsStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultDiagnosticsStatusNotificationHandler {
    pub fn new() -> Self {
        Self
    }
}

impl DiagnosticsStatusNotificationHandler for DefaultDiagnosticsStatusNotificationHandler {
    fn handle(&self, _req: DiagnosticsStatusNotificationRequest) {
    }
}