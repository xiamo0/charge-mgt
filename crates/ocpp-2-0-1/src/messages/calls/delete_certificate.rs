//! DeleteCertificate Request (Functional Block J)
//! 删除证书

use serde::{Deserialize, Serialize};

/// 证书哈希类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    /// 哈希算法: SHA256/SHA384/SHA512
    pub hash_algorithm: crate::common::HashAlgorithmEnumType,
    /// 发行者密钥名称哈希 (max 128)
    pub issuer_name_hash: String,
    /// 发行者密钥哈希 (max 128)
    pub issuer_key_hash: String,
    /// 证书序列号 (max 40)
    pub serial_number: String,
}

/// DeleteCertificate 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    /// 证书哈希数据
    pub certificate_hash_data: CertificateHashDataType,
}

impl DeleteCertificateRequest {
    pub fn new(certificate_hash_data: CertificateHashDataType) -> Self {
        Self {
            certificate_hash_data,
        }
    }
}

pub const ACTION: &str = "DeleteCertificate";
