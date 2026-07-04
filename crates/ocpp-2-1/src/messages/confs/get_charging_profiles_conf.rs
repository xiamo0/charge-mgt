//! GetChargingProfiles Confirmation (Block K)
use crate::common::{GetChargingProfilesStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesConfirmation {
    pub status: GetChargingProfilesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
