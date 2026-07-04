//! GetDERControl Request (Block R — 2.1 New)
use crate::common::DERControlEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<DERControlEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

pub const ACTION: &str = "GetDERControl";
