//! DeleteCertificate Request (Block M)
use serde::{Deserialize, Serialize};
use crate::common::CertificateHashDataType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    pub certificate_hash_data: CertificateHashDataType,
}

pub const ACTION: &str = "DeleteCertificate";
