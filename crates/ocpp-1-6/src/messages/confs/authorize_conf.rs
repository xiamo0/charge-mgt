//! Authorize 响应

use crate::common::id_tag::{AuthorizationStatus, IdTagInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizeConfirmation {
    pub status: AuthorizationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

impl AuthorizeConfirmation {
    pub fn accepted() -> Self {
        Self {
            status: AuthorizationStatus::Accepted,
            id_tag_info: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorize_confirmation_accepted() {
        let conf = AuthorizeConfirmation::accepted();
        assert_eq!(conf.status, AuthorizationStatus::Accepted);
        assert!(conf.id_tag_info.is_none());
    }

    #[test]
    fn test_authorize_confirmation_with_info() {
        let conf = AuthorizeConfirmation {
            status: AuthorizationStatus::Blocked,
            id_tag_info: Some(IdTagInfo::accepted()),
        };
        assert_eq!(conf.status, AuthorizationStatus::Blocked);
        assert!(conf.id_tag_info.is_some());
    }

    #[test]
    fn test_authorize_confirmation_skip_none() {
        let conf = AuthorizeConfirmation::accepted();
        let json = serde_json::to_string(&conf).unwrap();
        assert!(!json.contains("idTagInfo"));
    }

    #[test]
    fn test_authorize_confirmation_include_info() {
        let conf = AuthorizeConfirmation {
            status: AuthorizationStatus::Accepted,
            id_tag_info: Some(IdTagInfo::accepted()),
        };
        let json = serde_json::to_string(&conf).unwrap();
        let de: AuthorizeConfirmation = serde_json::from_str(&json).unwrap();
        assert!(de.id_tag_info.is_some());
    }

    #[test]
    fn test_authorize_confirmation_roundtrip() {
        let conf = AuthorizeConfirmation {
            status: AuthorizationStatus::Expired,
            id_tag_info: Some(IdTagInfo {
                status: AuthorizationStatus::Expired,
                expiry_date: Some("2024-12-31T23:59:59Z".to_string()),
                parent_id_tag: Some("PARENT001".to_string()),
            }),
        };
        let json = serde_json::to_string(&conf).unwrap();
        let de: AuthorizeConfirmation = serde_json::from_str(&json).unwrap();
        assert_eq!(conf.status, de.status);
        assert!(de.id_tag_info.is_some());
    }
}
