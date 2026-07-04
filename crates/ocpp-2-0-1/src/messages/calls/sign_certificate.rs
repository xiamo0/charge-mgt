//! SignCertificate Request (Functional Block J)
//! CSR 签名请求

use serde::{Deserialize, Serialize};

/// 证书签名用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
}

/// SignCertificate 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    /// CSR (PEM 格式, max 5500 chars)
    pub csr: String,
    /// 证书签名用途 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

impl SignCertificateRequest {
    pub fn new(csr: impl Into<String>) -> Self {
        Self {
            csr: csr.into(),
            certificate_type: None,
        }
    }

    pub fn with_type(mut self, certificate_type: CertificateSigningUseEnumType) -> Self {
        self.certificate_type = Some(certificate_type);
        self
    }
}

pub const ACTION: &str = "SignCertificate";
