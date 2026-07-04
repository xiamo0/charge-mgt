//! Reset Request (Block B)
use serde::{Deserialize, Serialize};
use crate::common::ResetEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    #[serde(rename = "type")]
    pub reset_type: ResetEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

pub const ACTION: &str = "Reset";
