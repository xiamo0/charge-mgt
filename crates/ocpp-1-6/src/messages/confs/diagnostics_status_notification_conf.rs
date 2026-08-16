//! DiagnosticsStatusNotification 响应
//!
//! 该消息为单向通知，OCPP 规范允许响应体为空。

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DiagnosticsStatusNotificationConfirmation;