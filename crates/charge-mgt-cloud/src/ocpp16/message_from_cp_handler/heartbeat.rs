use crate::ocpp16::entity::charge_point::{Column as CpColumn, Entity as ChargePoints};
use crate::ocpp16::message_from_cp_handler::Handler;
use chrono::Local;
use ocpp_1_6::calls::HeartbeatRequest;
use ocpp_1_6::confs::heartbeat_conf::HeartbeatConfirmation;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::log;

impl Handler<HeartbeatConfirmation> for HeartbeatRequest {
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<HeartbeatConfirmation, crate::ocpp16::error::HandlerError> {
        // 新 schema 的 charge_point 表没有独立的 last_heartbeat_at 列，
        // 仅在 update_time 中体现心跳时间戳。
        let now = Local::now().naive_local();

        if let Ok(db) = state.db() {
            ChargePoints::update_many()
                .col_expr(CpColumn::UpdateTime, Expr::value(now))
                .filter(CpColumn::ChargePointId.eq(&msg.charge_point_id))
                .filter(CpColumn::IsDeleted.eq(0_i16))
                .exec(db)
                .await?;
        } else {
            log::error!("db not exists");
        }

        let response = HeartbeatConfirmation::new(&chrono::Utc::now().to_rfc3339());

        Ok(response)
    }
}
