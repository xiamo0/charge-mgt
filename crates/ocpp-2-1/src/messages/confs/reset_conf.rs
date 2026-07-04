//! Reset Confirmation (Block B)
use serde::{Deserialize, Serialize};
use crate::common::{ResetStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetConfirmation {
    pub status: ResetStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
