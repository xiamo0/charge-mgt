//! DeleteCertificate Request (Block M)
use crate::common::CertificateHashDataType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    pub certificate_hash_data: CertificateHashDataType,
}

pub const ACTION: &str = "DeleteCertificate";
