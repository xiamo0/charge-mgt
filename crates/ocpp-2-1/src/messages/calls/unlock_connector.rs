//! UnlockConnector Request (Block F)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    pub evse_id: i32,
    pub connector_id: i32,
}

pub const ACTION: &str = "UnlockConnector";
