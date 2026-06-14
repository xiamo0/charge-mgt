use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::connector_status::ConnectorStatus;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_mgt_connectors_ocpp_1_6")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub charge_point_id: String,
    pub connector_id: i32,
    #[sea_orm(column_type = "custom(\"charge_mgt_connector_status\")")]
    pub status: ConnectorStatus,
    pub error_code: String,
    pub vendor_id: Option<String>,
    pub vendor_error_code: Option<String>,
    pub info: Option<String>,
    pub last_status_at: Option<DateTimeWithTimeZone>,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
