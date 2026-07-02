//! CertificateSigned Request (Functional Block J)
//! 下发签名证书

use serde::{Deserialize, Serialize};
use super::sign_certificate::CertificateSigningUseEnumType;

/// CertificateSigned 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    /// 证书链 (PEM 格式, max 10000 chars)
    pub certificate_chain: String,
    /// 证书签名用途 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

impl CertificateSignedRequest {
    pub fn new(certificate_chain: impl Into<String>) -> Self {
        Self {
            certificate_chain: certificate_chain.into(),
            certificate_type: None,
        }
    }

    pub fn with_type(mut self, certificate_type: CertificateSigningUseEnumType) -> Self {
        self.certificate_type = Some(certificate_type);
        self
    }
}

pub const ACTION: &str = "CertificateSigned";