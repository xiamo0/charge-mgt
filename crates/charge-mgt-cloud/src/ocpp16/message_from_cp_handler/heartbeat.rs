use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::error::AppError;
use crate::ocpp16::entity::charge_point::{Column as CpColumn, Entity as ChargePoints};
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_HEARTBEAT_CONFIRMATION;
use ocpp_1_6::calls::HeartbeatRequest;
use ocpp_1_6::confs::heartbeat_conf::HeartbeatConfirmation;

impl Handler<HeartbeatConfirmation> for HeartbeatRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<HeartbeatConfirmation, AppError> {
        let db = state.db()?;
        let now = Local::now().naive_local();

        // 新 schema 的 charge_point 表没有独立的 last_heartbeat_at 列，
        // 仅在 update_time 中体现心跳时间戳。
        ChargePoints::update_many()
            .col_expr(CpColumn::UpdateTime, sea_orm::sea_query::Expr::value(now))
            .filter(CpColumn::ChargePointId.eq(msg.charge_point_id.as_deref().unwrap_or("")))
            .filter(CpColumn::IsDeleted.eq(0_i16))
            .exec(db)
            .await?;

        let response = HeartbeatConfirmation::new(&chrono::Utc::now().to_rfc3339());
        Ok(response)
    }
}
