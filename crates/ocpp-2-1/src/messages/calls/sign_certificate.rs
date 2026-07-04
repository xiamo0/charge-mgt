//! SignCertificate Request (Block M)
use crate::common::CertificateSigningUseEnumType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    pub csr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

pub const ACTION: &str = "SignCertificate";
