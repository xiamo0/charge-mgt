//! Authorize Request (Block C)
use serde::{Deserialize, Serialize};
use crate::common::{IdTokenType, OCSPRequestDataType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub id_token: IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}

impl AuthorizeRequest {
    pub fn new(id_token: IdTokenType) -> Self {
        Self { id_token, certificate: None, iso_15118_certificate_hash_data: None }
    }
}

pub const ACTION: &str = "Authorize";
