//! SetNetworkProfile Request (Block B)
use crate::common::NetworkConnectionProfileType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    pub configuration_slot: i32,
    pub connection_data: NetworkConnectionProfileType,
}

pub const ACTION: &str = "SetNetworkProfile";
