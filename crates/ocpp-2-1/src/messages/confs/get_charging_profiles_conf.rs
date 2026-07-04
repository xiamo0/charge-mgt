//! GetChargingProfiles Confirmation (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{GetChargingProfilesStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesConfirmation {
    pub status: GetChargingProfilesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
