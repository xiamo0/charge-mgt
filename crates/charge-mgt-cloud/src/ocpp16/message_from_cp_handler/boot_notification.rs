use crate::ocpp16::entity::charge_point::{Column as ChargePointColumn, Entity as ChargePoints};
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use chrono::Utc;
use ocpp_1_6::calls::BootNotificationRequest;
use ocpp_1_6::confs::BootNotificationConfirmation;
use sea_orm::*;
use tracing::log;

impl Handler<BootNotificationConfirmation> for BootNotificationRequest {
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<BootNotificationConfirmation, crate::ocpp16::error::HandlerError> {
        let req: BootNotificationRequest = serde_json::from_value(msg.payload.clone())?;

        if req.charge_point_vendor.len() > 20 {
            return Err(HandlerError::FormationViolation(
                "chargePointVendor must be <= 20 characters".into(),
            ));
        }
        if req.charge_point_model.len() > 20 {
            return Err(HandlerError::FormationViolation(
                "chargePointModel must be <= 20 characters".into(),
            ));
        }

        let charge_point_serial_number = req.charge_point_serial_number.ok_or_else(|| {
            HandlerError::FormationViolation("charge_point_serial_number must not null".into())
        })?;
        let charge_box_serial_number = req.charge_box_serial_number.ok_or_else(|| {
            HandlerError::FormationViolation("charge_box_serial_number must not null".into())
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
                .ok_or_else(|| {
                    HandlerError::FormationViolation(
                        "charge_point_serial_number or charge_box_serial_number not exists".into(),
                    )
                })?;
        } else {
            log::error!("db not exists");
        }

        let confirmation = BootNotificationConfirmation::accepted(&Utc::now().to_rfc3339(), 30);

        Ok(confirmation)
    }
}

pub async fn handle(
    state: &AppState,
    msg: &CloudMessage,
) -> Result<serde_json::Value, HandlerError> {
    let req: BootNotificationRequest = serde_json::from_value(msg.payload.clone())?;

    if req.charge_point_vendor.len() > 20 {
        return Err(HandlerError::FormationViolation(
            "chargePointVendor must be <= 20 characters".into(),
        ));
    }
    if req.charge_point_model.len() > 20 {
        return Err(HandlerError::FormationViolation(
            "chargePointModel must be <= 20 characters".into(),
        ));
    }

    let charge_point_serial_number = req.charge_point_serial_number.ok_or_else(|| {
        HandlerError::FormationViolation("charge_point_serial_number must not null".into())
    })?;
    let charge_box_serial_number = req.charge_box_serial_number.ok_or_else(|| {
        HandlerError::FormationViolation("charge_box_serial_number must not null".into())
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
            .ok_or_else(|| {
                HandlerError::FormationViolation(
                    "charge_point_serial_number or charge_box_serial_number not exists".into(),
                )
            })?;
    } else {
        log::error!("db not exists");
    }

    let confirmation = BootNotificationConfirmation::accepted(&Utc::now().to_rfc3339(), 30);

    Ok(serde_json::to_value(&confirmation)?)
}
