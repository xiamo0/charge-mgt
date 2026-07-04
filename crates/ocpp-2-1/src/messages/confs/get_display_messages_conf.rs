//! GetDisplayMessages Confirmation (Block O)
use crate::common::{GetDisplayMessagesStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesConfirmation {
    pub status: GetDisplayMessagesStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
