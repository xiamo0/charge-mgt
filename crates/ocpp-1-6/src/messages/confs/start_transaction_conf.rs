//! StartTransaction 响应

use serde::{Deserialize, Serialize};
use crate::common::id_tag::IdTagInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartTransactionConfirmation {
    pub transaction_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}

impl StartTransactionConfirmation {
    pub fn new(transaction_id: i32) -> Self {
        Self {
            transaction_id,
            id_tag_info: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_transaction_confirmation_new() {
        let conf = StartTransactionConfirmation::new(42);
        assert_eq!(conf.transaction_id, 42);
        assert!(conf.id_tag_info.is_none());
    }

    #[test]
    fn test_start_transaction_confirmation_with_info() {
        let conf = StartTransactionConfirmation {
            transaction_id: 5,
            id_tag_info: Some(IdTagInfo::accepted()),
        };
        let json = serde_json::to_string(&conf).unwrap();
        let de: StartTransactionConfirmation = serde_json::from_str(&json).unwrap();
        assert!(de.id_tag_info.is_some());
    }

    #[test]
    fn test_start_transaction_confirmation_roundtrip() {
        let conf = StartTransactionConfirmation {
            transaction_id: 100,
            id_tag_info: Some(IdTagInfo {
                status: crate::common::id_tag::AuthorizationStatus::Accepted,
                expiry_date: Some("2024-12-31T23:59:59Z".to_string()),
                parent_id_tag: Some("PARENT001".to_string()),
            }),
        };
        let json = serde_json::to_string(&conf).unwrap();
        let de: StartTransactionConfirmation = serde_json::from_str(&json).unwrap();
        assert_eq!(conf.transaction_id, de.transaction_id);
        assert!(de.id_tag_info.is_some());
    }
}