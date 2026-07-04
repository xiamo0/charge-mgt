//! SendLocalList Confirmation (Block D)
use serde::{Deserialize, Serialize};
use crate::common::{SendLocalListStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListConfirmation {
    pub status: SendLocalListStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
