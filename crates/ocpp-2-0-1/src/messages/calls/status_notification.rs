//! StatusNotification Request (Functional Block F)
//! 上报 EVSE 上连接器的状态变化

use crate::common::ConnectorStatusEnumType;
use serde::{Deserialize, Serialize};

/// StatusNotification 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    /// 时间戳
    pub timestamp: String,
    /// 连接器状态
    pub connector_status: ConnectorStatusEnumType,
    /// EVSE ID (>0)
    pub evse_id: i32,
    /// 连接器 ID (>0)
    pub connector_id: i32,
}

impl StatusNotificationRequest {
    pub fn new(
        timestamp: impl Into<String>,
        connector_status: ConnectorStatusEnumType,
        evse_id: i32,
        connector_id: i32,
    ) -> Self {
        Self {
            timestamp: timestamp.into(),
            connector_status,
            evse_id,
            connector_id,
        }
    }
}

pub const ACTION: &str = "StatusNotification";
