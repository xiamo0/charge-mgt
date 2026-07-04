//! BootNotification Request (Block B)
use crate::common::{BootReasonEnumType, ChargingStationType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnumType,
    pub charging_station: ChargingStationType,
}

pub const ACTION: &str = "BootNotification";
