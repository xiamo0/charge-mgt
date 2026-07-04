//! ClearChargingProfile Confirmation (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{ClearChargingProfileStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileConfirmation {
    pub status: ClearChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
