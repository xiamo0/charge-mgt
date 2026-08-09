use crate::error::AppError;
use crate::ocpp16::entity::charge_point::{Column as ChargePointColumn, Entity as ChargePoints};
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use chrono::Utc;
use ocpp_1_6::ACTION_BOOT_NOTIFICATION_CONFIRMATION;
use ocpp_1_6::calls::BootNotificationRequest;
use ocpp_1_6::confs::BootNotificationConfirmation;
use sea_orm::*;
use tracing::log;

impl Handler<BootNotificationConfirmation> for BootNotificationRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<BootNotificationConfirmation, AppError> {
        let req: BootNotificationRequest = serde_json::from_value(msg.payload.clone())?;

        if req.charge_point_vendor.len() > 20 {
            return Err(AppError::OCPP_1_6_ERROR {
                action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.to_string(),
                detail: ">20".to_string(),
            });
        }
        if req.charge_point_model.len() > 20 {
            return Err(AppError::OCPP_1_6_ERROR {
                action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.to_string(),
                detail: "chargePointModel must be <= 20 characters".into(),
            });
        }

        let charge_point_serial_number =
            req.charge_point_serial_number
                .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.to_string(),
                    detail: "charge_point_serial_number must not null".to_string(),
                })?;
        let charge_box_serial_number =
            req.charge_box_serial_number
                .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.to_string(),
                    detail: "charge_box_serial_number must not null".to_string(),
                })?;

        if let Ok(db) = state.db() {
            ChargePoints::find()
                .filter(
                    ChargePointColumn::ChargeBoxSerialNumber
                        .eq(charge_box_serial_number.clone())
                        .and(
                            ChargePointColumn::ChargePointSerialNumber
                                .eq(charge_point_serial_number.clone()),
                        ),
                )
                .filter(ChargePointColumn::IsDeleted.eq(0_i16))
                .one(db)
                .await?
                .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.to_string(),
                    detail: "charge_point_serial_number or charge_box_serial_number not exists"
                        .to_string(),
                })?;
        } else {
            log::error!("db not exists");
        }

        let confirmation = BootNotificationConfirmation::accepted(&Utc::now().to_rfc3339(), 30);

        Ok(confirmation)
    }
}
