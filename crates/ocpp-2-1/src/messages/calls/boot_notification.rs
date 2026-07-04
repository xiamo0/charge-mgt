//! BootNotification Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::{BootReasonEnumType, ChargingStationType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnumType,
    pub charging_station: ChargingStationType,
}

pub const ACTION: &str = "BootNotification";
