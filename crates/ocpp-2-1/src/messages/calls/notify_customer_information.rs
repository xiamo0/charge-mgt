//! NotifyCustomerInformation Request (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    pub generated_at: String,
    pub request_id: i32,
}

pub const ACTION: &str = "NotifyCustomerInformation";
