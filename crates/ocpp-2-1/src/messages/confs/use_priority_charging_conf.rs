//! UsePriorityCharging Confirmation (Block K — 2.1)
use crate::common::{PriorityChargingStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingConfirmation {
    pub status: PriorityChargingStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
