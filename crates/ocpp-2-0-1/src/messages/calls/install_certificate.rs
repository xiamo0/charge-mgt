//! InstallCertificate Request (Functional Block J)
//! 安装 CA 根证书

use serde::{Deserialize, Serialize};

/// 证书用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
}

/// InstallCertificate 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    /// 证书用途类型
    pub certificate_type: InstallCertificateUseEnumType,
    /// 证书 (PEM 格式, max 5500 chars)
    pub certificate: String,
}

impl InstallCertificateRequest {
    pub fn new(certificate_type: InstallCertificateUseEnumType, certificate: impl Into<String>) -> Self {
        Self {
            certificate_type,
            certificate: certificate.into(),
        }
    }
}

pub const ACTION: &str = "InstallCertificate";