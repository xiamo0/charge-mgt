//! TriggerMessage Confirmation

use crate::common::{StatusInfoType, TriggerMessageStatusEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageConfirmation {
    pub status: TriggerMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl TriggerMessageConfirmation {
    pub fn new(status: TriggerMessageStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(TriggerMessageStatusEnumType::Accepted)
    }
}
