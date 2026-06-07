//! 授权列表类型
//!
//! 包含用于本地授权列表（Local Authorization List）的数据结构与版本枚举

use serde::{Deserialize, Serialize};

/// 本地授权列表的版本类型（Full / Differential）
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationVersion {
    Full,
    Differential,
}

/// 列表中的单项，表示一个 idTag 及其可选的 idTag 信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationList {
    /// idTag 字符串
    #[serde(rename = "idTag")]
    pub id_tag: String,
    /// 可选的 idTagInfo，包含授权状态 / 过期时间等
    #[serde(rename = "idTagInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<super::id_tag::IdTagInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // AuthorizationVersion 测试
    #[test]
    fn test_authorization_version_all_variants() {
        let variants = [
            AuthorizationVersion::Full,
            AuthorizationVersion::Differential,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: AuthorizationVersion = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_authorization_version_serialization() {
        assert_eq!(
            serde_json::to_string(&AuthorizationVersion::Full).unwrap(),
            "\"Full\""
        );
        assert_eq!(
            serde_json::to_string(&AuthorizationVersion::Differential).unwrap(),
            "\"Differential\""
        );
    }

    // AuthorizationList 测试
    #[test]
    fn test_authorization_list_minimal() {
        let list = AuthorizationList {
            id_tag: "TAG001".to_string(),
            id_tag_info: None,
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("idTag"));
        assert!(!json.contains("idTagInfo"));
        let de: AuthorizationList = serde_json::from_str(&json).unwrap();
        assert_eq!(list.id_tag, de.id_tag);
        assert!(de.id_tag_info.is_none());
    }

    #[test]
    fn test_authorization_list_with_info() {
        let list = AuthorizationList {
            id_tag: "TAG002".to_string(),
            id_tag_info: Some(super::super::id_tag::IdTagInfo::accepted()),
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("idTagInfo"));
        let de: AuthorizationList = serde_json::from_str(&json).unwrap();
        assert_eq!(list.id_tag, de.id_tag);
        assert!(de.id_tag_info.is_some());
    }

    #[test]
    fn test_authorization_list_roundtrip() {
        let list = AuthorizationList {
            id_tag: "TAG003".to_string(),
            id_tag_info: Some(super::super::id_tag::IdTagInfo {
                status: super::super::id_tag::AuthorizationStatus::Expired,
                expiry_date: Some("2024-12-31T23:59:59Z".to_string()),
                parent_id_tag: Some("PARENT001".to_string()),
            }),
        };
        let json = serde_json::to_string(&list).unwrap();
        let de: AuthorizationList = serde_json::from_str(&json).unwrap();
        assert_eq!(list.id_tag, de.id_tag);
        assert!(de.id_tag_info.is_some());
    }
}
