//! RequestStopTransaction Confirmation

use serde::{Deserialize, Serialize};
use crate::common::{RequestStartStopStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionConfirmation {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl RequestStopTransactionConfirmation {
    pub fn new(status: RequestStartStopStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(RequestStartStopStatusEnumType::Accepted)
    }
}