//! ReportChargingProfiles Request (Block K)
use crate::common::{ChargingLimitSourceEnumType, ChargingProfileType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    pub request_id: i32,
    pub charging_limit_source: ChargingLimitSourceEnumType,
    pub evse_id: i32,
    pub charging_profile: Vec<ChargingProfileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
}

pub const ACTION: &str = "ReportChargingProfiles";
