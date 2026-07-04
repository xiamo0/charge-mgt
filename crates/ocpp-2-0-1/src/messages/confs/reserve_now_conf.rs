//! ReserveNow Confirmation
use crate::common::ReserveNowStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowConfirmation {
    pub status: ReserveNowStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
