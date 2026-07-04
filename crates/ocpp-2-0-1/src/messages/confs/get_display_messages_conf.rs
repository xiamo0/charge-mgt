//! GetDisplayMessages Confirmation
use crate::common::response_status::GetDisplayMessagesStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesConfirmation {
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
