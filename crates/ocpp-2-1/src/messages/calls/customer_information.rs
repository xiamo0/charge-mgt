//! CustomerInformation Request (Block N)
use serde::{Deserialize, Serialize};
use crate::common::{CertificateHashDataType, IdTokenType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    pub request_id: i32,
    pub report: bool,
    pub clear: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<CertificateHashDataType>,
}

pub const ACTION: &str = "CustomerInformation";
