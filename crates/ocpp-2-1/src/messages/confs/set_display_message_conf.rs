//! SetDisplayMessage Confirmation (Block O)
use serde::{Deserialize, Serialize};
use crate::common::{DisplayMessageStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayMessageConfirmation {
    pub status: DisplayMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
