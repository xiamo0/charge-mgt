//! OCPP 2.0.1 / 2.1 Certificate management types (Functional Block M)

use serde::{Deserialize, Serialize};

/// 哈希算法枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum HashAlgorithmEnumType {
    SHA256,
    SHA384,
    SHA512,
}

/// OCSP 请求数据 (ISO 15118)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    pub issuer_key_hash: String,
    pub issuer_name_hash: String,
    pub responder_url: String,
    pub serial_number: String,
}

/// 证书哈希数据 (DeleteCertificate 等)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    pub issuer_name_hash: String,
    pub issuer_key_hash: String,
    pub serial_number: String,
}

/// 证书安装用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    CSMSRootCertificate,
    V2GCertificateChain,
    ManufacturerRootCertificate,
}

/// 证书签名用途枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
    ManufacturerCertificate,
}

/// CertificateHashDataChainType — GetInstalledCertificateIds 响应
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    pub certificate_type: InstallCertificateUseEnumType,
    pub certificate_hash_data: CertificateHashDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataChainType>>,
}

/// 证书状态来源枚举 (2.1 — 用于 GetCertificateChainStatus)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateStatusSourceEnumType {
    OCSP,
    CRL,
}

/// 证书状态枚举 (2.1 — 用于 GetCertificateChainStatus)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CertificateStatusEnumType {
    Good,
    Revoked,
    Unknown,
    Failed,
}

/// 证书状态请求信息 (2.1 — 用于 GetCertificateChainStatus)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusRequestInfoType {
    pub source: CertificateStatusSourceEnumType,
    pub certificate_hash_data: CertificateHashDataType,
    pub urls: Vec<String>,
}

/// 证书状态类型 (2.1 — GetCertificateChainStatus 响应字段)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusType {
    pub certificate_hash_data: CertificateHashDataType,
    pub source: CertificateStatusSourceEnumType,
    pub status: CertificateStatusEnumType,
    pub next_update: String,
}
