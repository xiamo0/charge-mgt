use chrono::Local;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, Set};

use crate::error::AppError;
use crate::ocpp16::entity::charge_transaction::ActiveModel;
use crate::ocpp16::entity::enums::{PaymentStatus, TransactionStatus};
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_START_TRANSACTION_CONFIRMATION;
use ocpp_1_6::calls::StartTransactionRequest;
use ocpp_1_6::confs::StartTransactionConfirmation;

impl Handler<StartTransactionConfirmation> for StartTransactionRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<StartTransactionConfirmation, AppError> {
        let req: StartTransactionRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        let db = state.db()?;
        let now = Local::now().naive_local();

        // 平台侧生成 transaction_id。i32 范围 → 用自纪元起的毫秒数，
        // 实际部署应当接入分布式 ID 分配服务（snowflake / segment）。
        let transaction_id = chrono::Utc::now().timestamp_millis() as i32;

        let meter_start_kwh = Decimal::from(req.meter_start) / Decimal::from(1000);

        let mut active: ActiveModel = Default::default();
        active.transaction_id = Set(transaction_id.to_string());
        active.tag_id = Set(req.id_tag);
        active.charge_point_id = Set(msg.charge_point_id.clone().unwrap_or_default());
        active.connector_id = Set(req.connector_id.to_string());
        active.status = Set(TransactionStatus::InProgress);
        active.start_time = Set(now);
        active.meter_start = Set(meter_start_kwh);
        active.payment_status = Set(PaymentStatus::Unpaid);
        active.is_offline_sync = Set(0);
        active.create_time = Set(now);
        active.update_time = Set(now);

        active.insert(db).await?;

        Ok(StartTransactionConfirmation::new(transaction_id))
    }
}
