//! BootNotification Request (Functional Block B)
//! 充电桩启动时向云平台注册

use serde::{Deserialize, Serialize};
use crate::common::{BootReasonEnumType, ChargingStationType};

/// BootNotification 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// 启动原因
    pub reason: BootReasonEnumType,
    /// 充电站信息
    pub charging_station: ChargingStationType,
}

impl BootNotificationRequest {
    pub fn new(reason: BootReasonEnumType, charging_station: ChargingStationType) -> Self {
        Self {
            reason,
            charging_station,
        }
    }
}

pub const ACTION: &str = "BootNotification";