//! IdTag 和授权类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AuthorizationStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IdTagInfo {
    pub status: AuthorizationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
}

impl IdTagInfo {
    pub fn accepted() -> Self {
        Self {
            status: AuthorizationStatus::Accepted,
            expiry_date: None,
            parent_id_tag: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AuthorizationStatus 测试
    #[test]
    fn test_authorization_status_all_variants() {
        let variants = [
            AuthorizationStatus::Accepted,
            AuthorizationStatus::Blocked,
            AuthorizationStatus::Expired,
            AuthorizationStatus::Invalid,
            AuthorizationStatus::ConcurrentTx,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: AuthorizationStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_authorization_status_accepted() {
        let status = AuthorizationStatus::Accepted;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Accepted\"");
    }

    // IdTagInfo 测试
    #[test]
    fn test_id_tag_info_accepted() {
        let info = IdTagInfo::accepted();
        assert_eq!(info.status, AuthorizationStatus::Accepted);
        assert!(info.expiry_date.is_none());
        assert!(info.parent_id_tag.is_none());
    }

    #[test]
    fn test_id_tag_info_with_expiry() {
        let info = IdTagInfo {
            status: AuthorizationStatus::Accepted,
            expiry_date: Some("2024-12-31T23:59:59Z".to_string()),
            parent_id_tag: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        let de: IdTagInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info.status, de.status);
        assert_eq!(info.expiry_date, de.expiry_date);
    }

    #[test]
    fn test_id_tag_info_skip_none_fields() {
        let info = IdTagInfo::accepted();
        let json = serde_json::to_string(&info).unwrap();
        assert!(!json.contains("expiryDate"));
        assert!(!json.contains("parentIdTag"));
    }

    #[test]
    fn test_id_tag_info_full() {
        let info = IdTagInfo {
            status: AuthorizationStatus::Expired,
            expiry_date: Some("2024-01-01T00:00:00Z".to_string()),
            parent_id_tag: Some("PARENT001".to_string()),
        };
        let json = serde_json::to_string(&info).unwrap();
        let de: IdTagInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info.status, de.status);
        assert_eq!(info.expiry_date, de.expiry_date);
        assert_eq!(info.parent_id_tag, de.parent_id_tag);
    }

    #[test]
    fn test_id_tag_info_roundtrip() {
        let info = IdTagInfo {
            status: AuthorizationStatus::Blocked,
            expiry_date: Some("2024-06-01T00:00:00Z".to_string()),
            parent_id_tag: Some("PARENT002".to_string()),
        };
        let json = serde_json::to_string(&info).unwrap();
        let de: IdTagInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, de);
    }
}
