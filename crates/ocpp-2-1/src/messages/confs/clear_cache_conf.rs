//! ClearCache Confirmation (Block C)
use serde::{Deserialize, Serialize};
use crate::common::{ClearCacheStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheConfirmation {
    pub status: ClearCacheStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
