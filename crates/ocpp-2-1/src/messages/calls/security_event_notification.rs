//! SecurityEventNotification Request (Block A)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    #[serde(rename = "type")]
    pub security_event_type: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

pub const ACTION: &str = "SecurityEventNotification";
