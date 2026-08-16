use chrono::{DateTime, Local, Utc};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::error::AppError;
use crate::ocpp16::entity::charge_transaction::{Column, Entity as Transactions, Model};
use crate::ocpp16::entity::enums::TransactionStatus;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_STOP_TRANSACTION_CONFIRMATION;
use ocpp_1_6::calls::StopTransactionRequest;
use ocpp_1_6::confs::StopTransactionConfirmation;

impl Handler<StopTransactionConfirmation> for StopTransactionRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<StopTransactionConfirmation, AppError> {
        let req: StopTransactionRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        let db = state.db()?;
        let tx_id_str = req.transaction_id.to_string();

        let existing = Transactions::find()
            .filter(Column::TransactionId.eq(tx_id_str.clone()))
            .one(db)
            .await?
            .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                action: ACTION_STOP_TRANSACTION_CONFIRMATION.into(),
                detail: format!(
                    "请求ID {}, transaction_id {tx_id_str} 不存在",
                    msg.unique_id.as_deref().unwrap_or("")
                ),
            })?;

        let Model {
            id,
            meter_start,
            meter_stop: _,
            total_energy: _,
            ..
        } = existing;

        let meter_stop_kwh = Decimal::from(req.meter_stop) / Decimal::from(1000);
        let total_energy = meter_stop_kwh - meter_start;
        let now = Local::now().naive_local();
        let end_time = parse_rfc3339_to_naive(&req.timestamp);

        let mut active: crate::ocpp16::entity::charge_transaction::ActiveModel = existing.into();
        active.status = Set(TransactionStatus::Completed);
        active.end_time = Set(Some(end_time));
        active.meter_stop = Set(Some(meter_stop_kwh));
        active.total_energy = Set(Some(total_energy));
        if let Some(reason) = req.reason.clone() {
            active.stop_reason = Set(Some(format!("{reason:?}")));
        }
        active.update_time = Set(now);

        let _ = id; // 主键由 DB 维护
        active.update(db).await?;

        Ok(StopTransactionConfirmation::default())
    }
}

fn parse_rfc3339_to_naive(ts: &str) -> chrono::NaiveDateTime {
    DateTime::parse_from_rfc3339(ts)
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|_| Utc::now().naive_utc())
}
