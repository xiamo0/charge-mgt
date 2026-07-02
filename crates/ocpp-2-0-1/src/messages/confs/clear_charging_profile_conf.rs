//! ClearChargingProfile Confirmation
use serde::{Deserialize, Serialize};
use crate::common::ClearChargingProfileStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileConfirmation {
    pub status: ClearChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
