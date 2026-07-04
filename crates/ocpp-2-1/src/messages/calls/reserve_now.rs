//! ReserveNow Request (Block H)
use serde::{Deserialize, Serialize};
use crate::common::{ConnectorEnumType, IdTokenType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    pub id: i32,
    pub expiry_date_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorEnumType>,
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

pub const ACTION: &str = "ReserveNow";
