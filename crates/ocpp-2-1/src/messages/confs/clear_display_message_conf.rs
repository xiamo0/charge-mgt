//! ClearDisplayMessage Confirmation (Block O)
use serde::{Deserialize, Serialize};
use crate::common::{ClearMessageStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageConfirmation {
    pub status: ClearMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
