//! EVSE and Connector Types

use serde::{Deserialize, Serialize};

/// EVSE (Electric Vehicle Supply Equipment) 类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

impl EVSEType {
    pub fn new(id: i32) -> Self {
        Self { id, connector_id: None }
    }

    pub fn with_connector(id: i32, connector_id: i32) -> Self {
        Self { id, connector_id: Some(connector_id) }
    }

    pub fn is_whole_station(&self) -> bool {
        self.id == 0
    }
}

/// 连接器类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_camel_case_types)]
pub enum ConnectorEnumType {
    CCCS1,
    CCCS2,
    CChaoJi,
    CGBT,
    CTesla,
    CType1,
    CType2,
    S309_1P_16A,
    S309_1P_32A,
    S309_3P_16A,
    S309_3P_32A,
    SBS1361,
    SCEE_7_7,
    SType1,
    SType2,
    Other1PhMax16A,
    Other1PhOver16A,
    Other3Ph,
    Pan,
    WInductive,
    WResonant,
    Undetermined,
    Unknown,
}
