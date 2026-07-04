//! UsePriorityCharging Confirmation (Block K — 2.1)
use serde::{Deserialize, Serialize};
use crate::common::{PriorityChargingStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingConfirmation {
    pub status: PriorityChargingStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
