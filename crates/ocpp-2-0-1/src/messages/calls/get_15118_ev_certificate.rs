//! Get15118EVCertificate Request (Functional Block J)
//! 获取 EV V2G 证书 (Plug & Charge)

use serde::{Deserialize, Serialize};

/// 证书操作枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateActionEnumType {
    Install,
    Update,
}

/// Get15118EVCertificate 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    /// ISO 15118 方案版本 (最长 50)
    pub iso_15118_schema_version: String,
    /// EXI 请求
    pub exi_request: String,
    /// 证书用途
    pub certificate_action: CertificateActionEnumType,
}

impl Get15118EVCertificateRequest {
    pub fn new(
        schema_version: impl Into<String>,
        exi_request: impl Into<String>,
        action: CertificateActionEnumType,
    ) -> Self {
        Self {
            iso_15118_schema_version: schema_version.into(),
            exi_request: exi_request.into(),
            certificate_action: action,
        }
    }
}

pub const ACTION: &str = "Get15118EVCertificate";
