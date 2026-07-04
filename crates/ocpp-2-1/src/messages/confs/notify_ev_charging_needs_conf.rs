//! NotifyEVChargingNeeds Confirmation (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{NotifyEVChargingNeedsStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsConfirmation {
    pub status: NotifyEVChargingNeedsStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
