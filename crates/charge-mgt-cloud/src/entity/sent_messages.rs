use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "charge_mgt_sent_messages_ocpp_1_6")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub unique_id: String,
    pub gateway_id: String,
    pub charge_point_id: String,
    pub direction: String,
    pub action: String,
    pub message_type: String,
    pub received_at: DateTimeWithTimeZone,
    pub processed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
