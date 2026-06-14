use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::entity::{charge_points::Column as CpColumn, ChargePoints};
use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(rename = "currentTime")]
    pub current_time: String,
}

pub async fn handle(state: &AppState, msg: &CloudMessage) -> Result<serde_json::Value, HandlerError> {
    let now: sea_orm::prelude::DateTimeWithTimeZone = Utc::now().into();

    ChargePoints::update_many()
        .col_expr(CpColumn::LastHeartbeatAt, Expr::value(now))
        .col_expr(CpColumn::OcppStatus, Expr::value("Online".to_string()))
        .col_expr(CpColumn::UpdatedAt, Expr::value(now))
        .filter(CpColumn::Id.eq(&msg.charge_point_id))
        .exec(&state.db)
        .await?;

    let response = Response {
        current_time: Utc::now().to_rfc3339(),
    };

    Ok(serde_json::to_value(&response)?)
}
