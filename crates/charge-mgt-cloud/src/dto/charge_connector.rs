use serde::Deserialize;

use crate::entity::enums::ConnectorType;

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChargeConnector {
    pub connector_id: Option<String>,
    pub connector_type: Option<ConnectorType>,
    pub status: Option<String>,
    pub error_code: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ChargeConnectorListQuery {
    #[serde(default)]
    pub charge_point_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ChargeConnectorListQuery {
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type ChargeConnectorResponse = crate::entity::charge_connector::Model;
