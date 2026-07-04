//! NotifyEVChargingNeeds Confirmation
use crate::common::response_status::NotifyEVChargingNeedsStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsConfirmation {
    pub status: NotifyEVChargingNeedsStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
