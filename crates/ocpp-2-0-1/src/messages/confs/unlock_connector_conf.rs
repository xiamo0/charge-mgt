//! UnlockConnector Confirmation

use serde::{Deserialize, Serialize};
use crate::common::{UnlockStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorConfirmation {
    pub status: UnlockStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl UnlockConnectorConfirmation {
    pub fn new(status: UnlockStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn unlocked() -> Self {
        Self::new(UnlockStatusEnumType::Unlocked)
    }
}