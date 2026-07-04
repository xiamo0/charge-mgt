//! VatNumberValidation Confirmation (Block I — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{AddressType, GenericStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationConfirmation {
    pub status: GenericStatusEnumType,
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<AddressType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}
