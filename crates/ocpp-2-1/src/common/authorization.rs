//! Authorization Types (Functional Block C / D)

use crate::common::IdTokenInfoType;
use serde::{Deserialize, Serialize};

/// 本地授权数据
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    pub id_token: crate::common::IdTokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
}

impl AuthorizationData {
    pub fn new(id_token: crate::common::IdTokenType) -> Self {
        Self {
            id_token,
            id_token_info: None,
        }
    }

    pub fn with_info(mut self, info: IdTokenInfoType) -> Self {
        self.id_token_info = Some(info);
        self
    }
}

/// 更新类型枚举 (SendLocalList)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateEnumType {
    Differential,
    Full,
}

/// 发送本地列表状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SendLocalListStatusEnumType {
    Accepted,
    Failed,
    VersionMismatch,
}
