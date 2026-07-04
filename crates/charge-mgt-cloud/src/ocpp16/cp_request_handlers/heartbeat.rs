use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::entity::{ChargePoints, charge_points::Column as CpColumn};
use crate::ocpp16::cp_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::HeartbeatRequest;
use ocpp_1_6::confs::heartbeat_conf::HeartbeatConfirmation;

impl Handler<HeartbeatConfirmation> for HeartbeatRequest {
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<HeartbeatConfirmation, crate::ocpp16::error::HandlerError> {
        let now: sea_orm::prelude::DateTimeWithTimeZone = Utc::now().into();

        ChargePoints::update_many()
            .col_expr(CpColumn::LastHeartbeatAt, Expr::value(now))
            .col_expr(CpColumn::OcppStatus, Expr::value("Online".to_string()))
            .col_expr(CpColumn::UpdatedAt, Expr::value(now))
            .filter(CpColumn::Id.eq(&msg.charge_point_id))
            .exec(&state.db)
            .await?;

        let response = HeartbeatConfirmation::new(&Utc::now().to_rfc3339());

        Ok(response)
    }
}
