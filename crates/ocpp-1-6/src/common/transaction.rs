//! 交易相关类型

use super::meter_value::MeterValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionData {
    #[serde(rename = "transactionData")]
    pub transaction_data: Vec<MeterValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Reason {
    EmergencyStop,
    EvDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    Reboot,
    Remote,
    SoftReset,
    StormReset,
    TillDisconnection,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reason 测试
    #[test]
    fn test_reason_all_variants() {
        let variants = [
            Reason::EmergencyStop,
            Reason::EvDisconnected,
            Reason::HardReset,
            Reason::Local,
            Reason::Other,
            Reason::PowerLoss,
            Reason::Reboot,
            Reason::Remote,
            Reason::SoftReset,
            Reason::StormReset,
            Reason::TillDisconnection,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: Reason = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }

    #[test]
    fn test_reason_serialization() {
        assert_eq!(
            serde_json::to_string(&Reason::EvDisconnected).unwrap(),
            "\"EvDisconnected\""
        );
        assert_eq!(
            serde_json::to_string(&Reason::PowerLoss).unwrap(),
            "\"PowerLoss\""
        );
        assert_eq!(serde_json::to_string(&Reason::Local).unwrap(), "\"Local\"");
    }

    #[test]
    fn test_reason_deserialization() {
        assert_eq!(
            serde_json::from_str::<Reason>("\"EmergencyStop\"").unwrap(),
            Reason::EmergencyStop
        );
        assert_eq!(
            serde_json::from_str::<Reason>("\"Remote\"").unwrap(),
            Reason::Remote
        );
        assert_eq!(
            serde_json::from_str::<Reason>("\"SoftReset\"").unwrap(),
            Reason::SoftReset
        );
    }

    // TransactionData 测试
    #[test]
    fn test_transaction_data_empty() {
        let td = TransactionData {
            transaction_data: vec![],
        };
        let json = serde_json::to_string(&td).unwrap();
        assert!(json.contains("transactionData"));
        let de: TransactionData = serde_json::from_str(&json).unwrap();
        assert!(de.transaction_data.is_empty());
    }

    #[test]
    fn test_transaction_data_with_meter_values() {
        let td = TransactionData {
            transaction_data: vec![MeterValue {
                timestamp: "2024-01-01T00:00:00Z".to_string(),
                sampled_value: vec![],
            }],
        };
        let json = serde_json::to_string(&td).unwrap();
        let de: TransactionData = serde_json::from_str(&json).unwrap();
        assert_eq!(td.transaction_data.len(), de.transaction_data.len());
    }

    #[test]
    fn test_transaction_data_roundtrip() {
        let td = TransactionData {
            transaction_data: vec![MeterValue {
                timestamp: "2024-01-01T12:00:00Z".to_string(),
                sampled_value: vec![super::super::meter_value::SampledValue {
                    value: "100.0".to_string(),
                    context: Some(super::super::meter_value::ReadingContext::TransactionBegin),
                    format: None,
                    measurand: None,
                    unit: None,
                }],
            }],
        };
        let json = serde_json::to_string(&td).unwrap();
        let de: TransactionData = serde_json::from_str(&json).unwrap();
        assert_eq!(td.transaction_data.len(), de.transaction_data.len());
    }
}
