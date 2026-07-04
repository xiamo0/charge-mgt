//! SendLocalList Request (Block D)
use crate::common::{AuthorizationData, UpdateEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    pub list_version: i32,
    pub update_type: UpdateEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
}

pub const ACTION: &str = "SendLocalList";
