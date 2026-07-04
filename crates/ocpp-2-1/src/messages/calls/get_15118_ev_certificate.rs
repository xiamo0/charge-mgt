//! Get15118EVCertificate Request (Block M)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    pub iso_15118_schema_version: String,
    pub exi_request: String,
}

pub const ACTION: &str = "Get15118EVCertificate";
