//! GetLog Confirmation
use crate::common::response_status::LogStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogConfirmation {
    pub status: LogStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
