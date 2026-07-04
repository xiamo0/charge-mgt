//! Authorize Request (Functional Block C)
//! 用户鉴权请求

use crate::common::{IdTokenType, OCSPRequestDataType};
use serde::{Deserialize, Serialize};

/// Authorize 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    /// 认证标识
    pub id_token: IdTokenType,
    /// X.509 PEM 证书 (可选, max 5500 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// ISO 15118 OCSP 证书哈希数据 (可选, max 4 个)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}

impl AuthorizeRequest {
    pub fn new(id_token: IdTokenType) -> Self {
        Self {
            id_token,
            certificate: None,
            iso_15118_certificate_hash_data: None,
        }
    }

    /// 添加证书 (用于 Plug & Charge)
    pub fn with_certificate(mut self, certificate: impl Into<String>) -> Self {
        self.certificate = Some(certificate.into());
        self
    }

    /// 添加 OCSP 证书哈希数据
    pub fn with_certificate_hash_data(mut self, hash_data: Vec<OCSPRequestDataType>) -> Self {
        self.iso_15118_certificate_hash_data = Some(hash_data);
        self
    }
}

pub const ACTION: &str = "Authorize";
