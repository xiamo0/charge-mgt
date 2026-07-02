//! CustomerInformation Confirmation
use serde::{Deserialize, Serialize};
use crate::common::response_status::CustomerInformationStatusEnumType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationConfirmation {
    pub status: CustomerInformationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<crate::common::StatusInfoType>,
}
