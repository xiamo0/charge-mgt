//! ClearCache Confirmation

use crate::common::{ClearCacheStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheConfirmation {
    pub status: ClearCacheStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl ClearCacheConfirmation {
    pub fn new(status: ClearCacheStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(ClearCacheStatusEnumType::Accepted)
    }
}
