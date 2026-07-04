//! RequestStartTransaction Request (Block F)
use serde::{Deserialize, Serialize};
use crate::common::{ChargingProfileType, IdTokenType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    pub id_token: IdTokenType,
    pub remote_start_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType>,
}

pub const ACTION: &str = "RequestStartTransaction";
