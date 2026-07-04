//! SetDisplayMessage Request (Block O)
use serde::{Deserialize, Serialize};
use crate::common::MessageInfoType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageRequest {
    pub message: MessageInfoType,
}

pub const ACTION: &str = "SetDisplayMessage";
