//! IdToken and Authorization Types (Functional Block C)

use serde::{Deserialize, Serialize};

/// IdToken 类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum IdTokenEnumType {
    /// 中央系统标识
    Central,
    /// e-Mobility Account Identifier
    EMAID,
    /// ISO 14443 RFID
    ISO14443,
    /// ISO 15693 RFID
    ISO15693,
    /// 密码/PIN
    KeyCode,
    /// 本地标识
    Local,
    /// MAC地址
    MacAddress,
    /// 无需授权
    NoAuthorization,
}

/// IdToken 附加信息
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    /// 附加令牌标识
    pub additional_id_token: String,
    /// 令牌类型
    #[serde(rename = "type")]
    pub token_type: String,
}

/// IdToken 结构
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// 令牌值 (max 36 chars)
    pub id_token: String,
    /// 令牌类型
    #[serde(rename = "type")]
    pub token_type: IdTokenEnumType,
    /// 附加信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
}

impl IdTokenType {
    /// 创建新的 IdToken
    pub fn new(id_token: impl Into<String>, token_type: IdTokenEnumType) -> Self {
        Self {
            id_token: id_token.into(),
            token_type,
            additional_info: None,
        }
    }

    /// 带附加信息的 IdToken
    pub fn with_additional_info(mut self, additional_info: Vec<AdditionalInfoType>) -> Self {
        self.additional_info = Some(additional_info);
        self
    }
}

/// 授权状态枚举 (Functional Block C)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationStatusEnumType {
    /// 接受
    Accepted,
    /// 锁定
    Blocked,
    /// 并发事务
    ConcurrentTx,
    /// 已过期
    Expired,
    /// 无效
    Invalid,
    /// 无余额
    NoCredit,
    /// EVSE类型不允许
    NotAllowedTypeEVSE,
    /// 不在此位置
    NotAtThisLocation,
    /// 不在此时间
    NotAtThisTime,
    /// 未知
    Unknown,
}

/// 消息内容格式
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageFormatEnumType {
    /// ASCII
    ASCII,
    /// HTML
    HTML,
    /// URI
    URI,
    /// UTF8
    UTF8,
}

/// 消息内容类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    /// 内容格式
    pub format: MessageFormatEnumType,
    /// 消息内容
    pub content: String,
    /// 语言标识 (可选, 最长8字符)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl MessageContentType {
    /// 创建新的消息内容
    pub fn new(format: MessageFormatEnumType, content: impl Into<String>) -> Self {
        Self {
            format,
            content: content.into(),
            language: None,
        }
    }

    /// 带语言的消息内容
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
}

/// IdToken 授权信息
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    /// 授权状态
    pub status: AuthorizationStatusEnumType,
    /// 缓存过期时间 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<String>,
    /// 充电优先级 (-9 到 9, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    /// 语言1 (可选, 最长8字符)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    /// 语言2 (可选, 最长8字符)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    /// 组令牌 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    /// 个人消息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
    /// 证书状态信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
}

impl IdTokenInfoType {
    /// 创建 Accepted 状态的 IdTokenInfo
    pub fn accepted() -> Self {
        Self {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
            certificate_status: None,
        }
    }

    /// 创建 Rejected 状态的 IdTokenInfo
    pub fn rejected() -> Self {
        Self {
            status: AuthorizationStatusEnumType::Invalid,
            cache_expiry_date_time: None,
            charging_priority: None,
            language1: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
            certificate_status: None,
        }
    }
}

/// 授权证书状态枚举 (ISO 15118 Plug & Charge)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizeCertificateStatusEnumType {
    /// 接受
    Accepted,
    /// 签名错误
    SignatureError,
    /// 证书过期
    CertificateExpired,
    /// 证书吊销
    CertificateRevoked,
    /// 无可用证书
    NoCertificateAvailable,
    /// 证书链错误
    CertChainError,
    /// 合约取消
    ContractCancelled,
}

/// 哈希算法枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum HashAlgorithmEnumType {
    /// SHA-256
    SHA256,
    /// SHA-384
    SHA384,
    /// SHA-512
    SHA512,
}

/// OCSP 请求数据 (ISO 15118)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    /// 哈希算法
    pub hash_algorithm: HashAlgorithmEnumType,
    /// 发行者公钥哈希
    pub issuer_key_hash: String,
    /// 发行者DN哈希
    pub issuer_name_hash: String,
    /// 响应者URL
    pub responder_url: String,
    /// 证书序列号
    pub serial_number: String,
}

/// CertificateHashDataChainType - 证书哈希链 (用于 GetInstalledCertificateIds 响应)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    pub certificate_type: InstallCertificateUseEnumType,
    pub certificate_hash_data: OCSPRequestDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataChainType>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_token_enum_serialization() {
        assert_eq!(
            serde_json::to_string(&IdTokenEnumType::ISO14443).unwrap(),
            "\"ISO14443\""
        );
        assert_eq!(
            serde_json::to_string(&IdTokenEnumType::ISO15693).unwrap(),
            "\"ISO15693\""
        );
        assert_eq!(
            serde_json::to_string(&IdTokenEnumType::EMAID).unwrap(),
            "\"EMAID\""
        );
    }

    #[test]
    fn test_id_token_type() {
        let token = IdTokenType::new("ABC123456789", IdTokenEnumType::ISO14443);
        let json = serde_json::to_string(&token).unwrap();
        let de: IdTokenType = serde_json::from_str(&json).unwrap();
        assert_eq!(token, de);
        assert_eq!(token.id_token, "ABC123456789");
        assert_eq!(token.token_type, IdTokenEnumType::ISO14443);
    }

    #[test]
    fn test_authorization_status_enum() {
        let variants = [
            AuthorizationStatusEnumType::Accepted,
            AuthorizationStatusEnumType::Blocked,
            AuthorizationStatusEnumType::ConcurrentTx,
            AuthorizationStatusEnumType::Expired,
            AuthorizationStatusEnumType::Invalid,
            AuthorizationStatusEnumType::NoCredit,
            AuthorizationStatusEnumType::NotAllowedTypeEVSE,
            AuthorizationStatusEnumType::NotAtThisLocation,
            AuthorizationStatusEnumType::NotAtThisTime,
            AuthorizationStatusEnumType::Unknown,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: AuthorizationStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_id_token_info_type_accepted() {
        let info = IdTokenInfoType::accepted();
        assert_eq!(info.status, AuthorizationStatusEnumType::Accepted);
        let json = serde_json::to_string(&info).unwrap();
        let de: IdTokenInfoType = serde_json::from_str(&json).unwrap();
        assert_eq!(info, de);
    }

    #[test]
    fn test_message_content_type() {
        let msg = MessageContentType::new(MessageFormatEnumType::UTF8, "欢迎充电");
        let json = serde_json::to_string(&msg).unwrap();
        let de: MessageContentType = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, de);
    }
}
