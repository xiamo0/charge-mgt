//! ClearDisplayMessage Request (Block O)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    pub id: i32,
}

pub const ACTION: &str = "ClearDisplayMessage";
