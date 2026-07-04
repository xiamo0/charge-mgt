//! ReportChargingProfiles Request (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{ChargingLimitSourceEnumType, ChargingProfileType};

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
