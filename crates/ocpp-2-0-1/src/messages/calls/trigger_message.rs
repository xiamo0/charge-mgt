//! TriggerMessage Request (Functional Block B)
//! 触发充电桩发送特定消息

use serde::{Deserialize, Serialize};
use crate::common::{EVSEType};

/// 消息触发类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageTriggerEnumType {
    BootNotification,
    LogStatusNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    SignChargingStationCertificate,
    SignV2GCertificate,
    StatusNotification,
    TransactionEvent,
    SignCombinedCertificate,
    PublishFirmwareStatusNotification,
}

/// TriggerMessage 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    /// 请求的消息类型
    pub requested_message: MessageTriggerEnumType,
    /// EVSE (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

impl TriggerMessageRequest {
    pub fn new(requested_message: MessageTriggerEnumType) -> Self {
        Self {
            requested_message,
            evse: None,
        }
    }

    /// 指定 EVSE
    pub fn for_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }
}

pub const ACTION: &str = "TriggerMessage";