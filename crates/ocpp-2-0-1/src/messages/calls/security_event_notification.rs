//! SecurityEventNotification Request (Functional Block A)
//! 上报安全相关事件

use serde::{Deserialize, Serialize};

/// 安全事件通知请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    /// 事件类型 (max 50 chars)
    #[serde(rename = "type")]
    pub event_type: String,
    /// 时间戳
    pub timestamp: String,
    /// 技术信息 (可选, max 255 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

impl SecurityEventNotificationRequest {
    pub fn new(event_type: impl Into<String>, timestamp: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            timestamp: timestamp.into(),
            tech_info: None,
        }
    }

    /// 添加技术信息
    pub fn with_tech_info(mut self, tech_info: impl Into<String>) -> Self {
        self.tech_info = Some(tech_info.into());
        self
    }
}

pub const ACTION: &str = "SecurityEventNotification";