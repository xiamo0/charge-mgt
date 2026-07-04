//! Authorization Types (Functional Block C)

use crate::common::IdTokenInfoType;
use serde::{Deserialize, Serialize};

/// 本地授权数据
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// ID令牌
    pub id_token: crate::common::IdTokenType,
    /// 令牌信息 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
}

impl AuthorizationData {
    /// 创建新的授权数据
    pub fn new(id_token: crate::common::IdTokenType) -> Self {
        Self {
            id_token,
            id_token_info: None,
        }
    }

    /// 带令牌信息的授权数据
    pub fn with_info(mut self, info: IdTokenInfoType) -> Self {
        self.id_token_info = Some(info);
        self
    }
}

/// 更新类型枚举 (SendLocalList)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateEnumType {
    /// 增量更新
    Differential,
    /// 全量更新
    Full,
}

/// 发送本地列表状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SendLocalListStatusEnumType {
    /// 已接受
    Accepted,
    /// 失败
    Failed,
    /// 版本不匹配
    VersionMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{IdTokenEnumType, IdTokenType};

    #[test]
    fn test_authorization_data() {
        let token = IdTokenType::new("ABC123", IdTokenEnumType::ISO14443);
        let auth_data = AuthorizationData::new(token);
        assert!(auth_data.id_token_info.is_none());

        let json = serde_json::to_string(&auth_data).unwrap();
        let de: AuthorizationData = serde_json::from_str(&json).unwrap();
        assert_eq!(auth_data, de);
    }

    #[test]
    fn test_authorization_data_with_info() {
        let token = IdTokenType::new("ABC123", IdTokenEnumType::ISO14443);
        let info = IdTokenInfoType::accepted();
        let auth_data = AuthorizationData::new(token).with_info(info);
        assert!(auth_data.id_token_info.is_some());
    }

    #[test]
    fn test_update_enum() {
        let variants = [UpdateEnumType::Differential, UpdateEnumType::Full];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: UpdateEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_send_local_list_status() {
        let variants = [
            SendLocalListStatusEnumType::Accepted,
            SendLocalListStatusEnumType::Failed,
            SendLocalListStatusEnumType::VersionMismatch,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: SendLocalListStatusEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}
