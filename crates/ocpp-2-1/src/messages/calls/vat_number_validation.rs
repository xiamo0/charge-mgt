//! VatNumberValidation Request (Block I — 2.1 New)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationRequest {
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

pub const ACTION: &str = "VatNumberValidation";
