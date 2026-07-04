//! InstallCertificate Request (Block M)
use serde::{Deserialize, Serialize};
use crate::common::InstallCertificateUseEnumType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    pub certificate_type: InstallCertificateUseEnumType,
    pub certificate: String,
}

pub const ACTION: &str = "InstallCertificate";
