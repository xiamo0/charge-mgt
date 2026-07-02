//! GetDisplayMessages Confirmation
use serde::{Deserialize, Serialize};
use crate::common::response_status::GetDisplayMessagesStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesConfirmation {
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
