//! StatusNotification Request (Block G)
use serde::{Deserialize, Serialize};
use crate::common::ConnectorStatusEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub timestamp: String,
    pub connector_status: ConnectorStatusEnumType,
    pub evse_id: i32,
    pub connector_id: i32,
}

pub const ACTION: &str = "StatusNotification";
