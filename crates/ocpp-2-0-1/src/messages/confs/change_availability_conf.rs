//! ChangeAvailability Confirmation

use crate::common::{ChangeAvailabilityStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityConfirmation {
    pub status: ChangeAvailabilityStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl ChangeAvailabilityConfirmation {
    pub fn new(status: ChangeAvailabilityStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(ChangeAvailabilityStatusEnumType::Accepted)
    }
}
