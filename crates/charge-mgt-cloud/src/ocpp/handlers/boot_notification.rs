use crate::entity::ChargePointColumn;
use crate::entity::ChargePoints;
use crate::ocpp::envelope::CloudMessage;
use crate::ocpp::error::HandlerError;
use crate::state::AppState;
use chrono::Utc;
use ocpp_1_6::calls::BootNotificationRequest;
use ocpp_1_6::confs::BootNotificationConfirmation;
use sea_orm::*;
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
    //判断表中是否有相同的 charge_point_serial_number 或 charge_box_serial_number
    //如果没有 返回 Err(HandlerError::FormationViolation("charge_point_serial_number or charge_box_serial_number already exists".into()))

    ChargePoints::find()
        .filter(
            ChargePointColumn::ChargeBoxSerial
                .eq(charge_box_serial_number.clone())
                .and(ChargePointColumn::SerialNumber.eq(charge_point_serial_number.clone())),
        )
        .one(&state.db)
        .await?
        .ok_or_else(|| {
            HandlerError::FormationViolation(
                "charge_point_serial_number or charge_box_serial_number not exists".into(),
            )
        })?;

    let confirmation = BootNotificationConfirmation::accepted(&Utc::now().to_rfc3339(), 30);

    Ok(serde_json::to_value(&confirmation)?)
}
