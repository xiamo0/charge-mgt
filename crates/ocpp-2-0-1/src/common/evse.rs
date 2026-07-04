//! EVSE and Connector Types (Functional Block B - Provisioning)

use serde::{Deserialize, Serialize};

/// EVSE (Electric Vehicle Supply Equipment) 类型
/// 表示一个独立的充电设备
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    /// EVSE ID (>0)
    pub id: i32,
    /// 连接器 ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

impl EVSEType {
    /// 创建新的 EVSE 实例
    pub fn new(id: i32) -> Self {
        Self {
            id,
            connector_id: None,
        }
    }

    /// 创建带连接器的 EVSE 实例
    pub fn with_connector(id: i32, connector_id: i32) -> Self {
        Self {
            id,
            connector_id: Some(connector_id),
        }
    }

    /// 检查是否为整个充电桩（id=0）
    pub fn is_whole_station(&self) -> bool {
        self.id == 0
    }
}

/// 连接器类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_camel_case_types)]
pub enum ConnectorEnumType {
    /// cCCS1
    CCCS1,
    /// cCCS2
    CCCS2,
    /// cChaoJi
    CChaoJi,
    /// cGBT
    CGBT,
    /// cTesla
    CTesla,
    /// cType1
    CType1,
    /// cType2
    CType2,
    /// s309_1P_16A
    S309_1P_16A,
    /// s309_1P_32A
    S309_1P_32A,
    /// s309_3P_16A
    S309_3P_16A,
    /// s309_3P_32A
    S309_3P_32A,
    /// sBS1361
    SBS1361,
    /// sCEE_7_7
    SCEE_7_7,
    /// sType1
    SType1,
    /// sType2
    SType2,
    /// Other1PhMax16A
    Other1PhMax16A,
    /// Other1PhOver16A
    Other1PhOver16A,
    /// Other3Ph
    Other3Ph,
    /// Pan
    Pan,
    /// wInductive
    WInductive,
    /// wResonant
    WResonant,
    /// Undetermined
    Undetermined,
    /// Unknown
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evse_type_serialization() {
        let evse = EVSEType::new(1);
        let json = serde_json::to_string(&evse).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(!json.contains("connectorId")); // skip_serializing_if = None

        let de: EVSEType = serde_json::from_str(&json).unwrap();
        assert_eq!(evse, de);
    }

    #[test]
    fn test_evse_type_with_connector() {
        let evse = EVSEType::with_connector(1, 2);
        let json = serde_json::to_string(&evse).unwrap();
        assert!(json.contains("\"connectorId\":2"));

        let de: EVSEType = serde_json::from_str(&json).unwrap();
        assert_eq!(evse, de);
    }

    #[test]
    fn test_evse_is_whole_station() {
        let whole_station = EVSEType::new(0);
        assert!(whole_station.is_whole_station());

        let specific_evse = EVSEType::new(1);
        assert!(!specific_evse.is_whole_station());
    }

    #[test]
    fn test_connector_enum_type() {
        let variants = [
            ConnectorEnumType::CCCS1,
            ConnectorEnumType::CCCS2,
            ConnectorEnumType::CType1,
            ConnectorEnumType::CType2,
            ConnectorEnumType::SType2,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let de: ConnectorEnumType = serde_json::from_str(&json).unwrap();
            assert_eq!(v, de);
        }
    }
}
