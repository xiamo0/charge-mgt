//! BootNotification Confirmation (Functional Block B)

use serde::{Deserialize, Serialize};
use crate::common::{RegistrationStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationConfirmation {
    pub status: RegistrationStatusEnumType,
    pub current_time: String,
    pub interval: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl BootNotificationConfirmation {
    pub fn new(
        status: RegistrationStatusEnumType,
        current_time: impl Into<String>,
        interval: i32,
    ) -> Self {
        Self {
            status,
            current_time: current_time.into(),
            interval,
            status_info: None,
        }
    }

    pub fn accepted(current_time: impl Into<String>, interval: i32) -> Self {
        Self::new(RegistrationStatusEnumType::Accepted, current_time, interval)
    }
}