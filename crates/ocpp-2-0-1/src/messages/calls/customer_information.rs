//! CustomerInformation Request (Functional Block G)
//! 请求或清除客户数据（GDPR）

use serde::{Deserialize, Serialize};
use crate::common::IdTokenType;

/// CustomerInformation 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 是否报告
    pub report: bool,
    /// 是否清除
    pub clear: bool,
    /// 客户标识符 (可选, max 64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    /// ID Token (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    /// 客户证书 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<super::delete_certificate::CertificateHashDataType>,
}

impl CustomerInformationRequest {
    pub fn new(request_id: i32, report: bool, clear: bool) -> Self {
        Self {
            request_id,
            report,
            clear,
            customer_identifier: None,
            id_token: None,
            customer_certificate: None,
        }
    }

    pub fn for_customer(mut self, id: impl Into<String>) -> Self {
        self.customer_identifier = Some(id.into());
        self
    }

    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }
}

pub const ACTION: &str = "CustomerInformation";