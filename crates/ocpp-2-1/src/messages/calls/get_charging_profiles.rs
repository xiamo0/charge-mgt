//! GetChargingProfiles Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::ChargingProfileCriterionType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    pub charging_profile: ChargingProfileCriterionType,
}

pub const ACTION: &str = "GetChargingProfiles";
