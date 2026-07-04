//! TriggerMessage Request (Block F)
use serde::{Deserialize, Serialize};
use crate::common::{EVSEType, MessageTriggerEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageRequest {
    pub requested_message: MessageTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

pub const ACTION: &str = "TriggerMessage";
