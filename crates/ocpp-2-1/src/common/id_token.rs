//! IdToken and Authorization Types (Functional Block C)

use serde::{Deserialize, Serialize};

/// IdToken 类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum IdTokenEnumType {
    Central,
    EMAID,
    ISO14443,
    ISO15693,
    KeyCode,
    Local,
    MacAddress,
    NoAuthorization,
}

/// IdToken 附加信息
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    pub additional_id_token: String,
    #[serde(rename = "type")]
    pub token_type: String,
}

/// IdToken 结构
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    pub id_token: String,
    #[serde(rename = "type")]
    pub token_type: IdTokenEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<Vec<AdditionalInfoType>>,
}

impl IdTokenType {
    pub fn new(id_token: impl Into<String>, token_type: IdTokenEnumType) -> Self {
        Self {
            id_token: id_token.into(),
            token_type,
            additional_info: None,
        }
    }

    pub fn with_additional_info(mut self, additional_info: Vec<AdditionalInfoType>) -> Self {
        self.additional_info = Some(additional_info);
        self
    }
}

/// 授权状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationStatusEnumType {
    Accepted,
    Blocked,
    ConcurrentTx,
    Expired,
    Invalid,
    NoCredit,
    NotAllowedTypeEVSE,
    NotAtThisLocation,
    NotAtThisTime,
    Unknown,
}

/// 消息内容格式
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageFormatEnumType {
    ASCII,
    HTML,
    URI,
    UTF8,
}

/// 消息内容类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    pub format: MessageFormatEnumType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl MessageContentType {
    pub fn new(format: MessageFormatEnumType, content: impl Into<String>) -> Self {
        Self {
            format,
            content: content.into(),
            language: None,
        }
    }
}

/// IdToken 授权信息
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    pub status: AuthorizationStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
}

impl IdTokenInfoType {
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
    Accepted,
    SignatureError,
    CertificateExpired,
    CertificateRevoked,
    NoCertificateAvailable,
    CertChainError,
    ContractCancelled,
}
