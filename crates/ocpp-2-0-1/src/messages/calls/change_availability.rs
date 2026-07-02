//! ChangeAvailability Request (Functional Block F)
//! 变更 EVSE 或连接器可用性

use serde::{Deserialize, Serialize};
use crate::common::{EVSEType, OperationalStatusEnumType};

/// ChangeAvailability 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    /// 操作状态: Operative/Inoperative
    pub operational_status: OperationalStatusEnumType,
    /// EVSE (可选, 不指定则影响整个充电站)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

impl ChangeAvailabilityRequest {
    pub fn new(operational_status: OperationalStatusEnumType) -> Self {
        Self {
            operational_status,
            evse: None,
        }
    }

    /// 指定 EVSE
    pub fn for_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }
}

pub const ACTION: &str = "ChangeAvailability";