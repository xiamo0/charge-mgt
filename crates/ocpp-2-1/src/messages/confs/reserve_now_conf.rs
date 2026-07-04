//! ReserveNow Confirmation (Block H)
use serde::{Deserialize, Serialize};
use crate::common::{ReserveNowStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowConfirmation {
    pub status: ReserveNowStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
