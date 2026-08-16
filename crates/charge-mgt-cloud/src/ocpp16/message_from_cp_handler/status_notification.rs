use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::error::AppError;
use crate::ocpp16::entity::charge_connector::{
    ActiveModel, Column as ConnectorColumn, Entity as Connectors,
};
use crate::ocpp16::entity::enums::ConnectorType;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_STATUS_NOTIFICATION_CONFIRMATION;
use ocpp_1_6::calls::StatusNotificationRequest;
use ocpp_1_6::confs::StatusNotificationConfirmation;

impl Handler<StatusNotificationConfirmation> for StatusNotificationRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<StatusNotificationConfirmation, AppError> {
        let req: StatusNotificationRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        let db = state.db()?;
        let connector_id = req.connector_id.to_string();
        let status_str = serde_json::to_value(req.status)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".to_string());
        let error_code_str = serde_json::to_value(req.error_code)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "NoError".to_string());
        let now = Local::now().naive_local();

        let existing = Connectors::find()
            .filter(ConnectorColumn::ChargePointId.eq(msg.charge_point_id.clone()))
            .filter(ConnectorColumn::ConnectorId.eq(connector_id.clone()))
            .one(db)
            .await?;

        let active = match existing {
            Some(model) => {
                let mut a: ActiveModel = model.into();
                a.status = Set(status_str);
                a.error_code = Set(Some(error_code_str));
                a.last_heartbeat_time = Set(Some(now));
                a.update_time = Set(now);
                a
            }
            None => {
                // 桩首次上报某连接器 → on-demand 创建（无 HTTP 创建端点）。
                // connector_type 占位为 GbtDc，可通过 PATCH 修正。
                let mut a: ActiveModel = Default::default();
                a.charge_point_id = Set(msg.charge_point_id.clone().unwrap_or_default());
                a.connector_id = Set(connector_id);
                a.connector_type = Set(ConnectorType::GbtDc);
                a.status = Set(status_str);
                a.error_code = Set(Some(error_code_str));
                a.last_heartbeat_time = Set(Some(now));
                a.create_time = Set(now);
                a.update_time = Set(now);
                a
            }
        };

        active.save(db).await?;

        Ok(StatusNotificationConfirmation)
    }
}
