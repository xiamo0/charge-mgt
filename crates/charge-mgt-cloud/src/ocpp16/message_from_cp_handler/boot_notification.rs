use chrono::{Local, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::error::AppError;
use crate::ocpp16::entity::charge_point::{
    ActiveModel, Column as ChargePointColumn, Entity as ChargePoints,
};
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_BOOT_NOTIFICATION_CONFIRMATION;
use ocpp_1_6::calls::BootNotificationRequest;
use ocpp_1_6::confs::BootNotificationConfirmation;

impl Handler<BootNotificationConfirmation> for BootNotificationRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<BootNotificationConfirmation, AppError> {
        let req: BootNotificationRequest =
            serde_json::from_value(msg.payload.clone().unwrap_or(serde_json::Value::Null))?;

        if req.charge_point_vendor.len() > 20 {
            return Err(AppError::OCPP_1_6_ERROR {
                action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.into(),
                detail: format!(
                    "请求ID {}, chargePointVendor must be <= 20 characters",
                    msg.unique_id.as_deref().unwrap_or("")
                ),
            });
        }
        if req.charge_point_model.len() > 20 {
            return Err(AppError::OCPP_1_6_ERROR {
                action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.into(),
                detail: format!(
                    "请求ID {}, chargePointModel must be <= 20 characters",
                    msg.unique_id.as_deref().unwrap_or("")
                ),
            });
        }

        // 必须先有 serial 才能匹配预注册记录（boot 流程要求桩预注册过）
        let charge_point_serial_number =
            req.charge_point_serial_number
                .clone()
                .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {}, charge_point_serial_number must not null",
                        msg.unique_id.as_deref().unwrap_or("")
                    ),
                })?;
        let charge_box_serial_number =
            req.charge_box_serial_number
                .clone()
                .ok_or_else(|| AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {}, charge_box_serial_number must not null",
                        msg.unique_id.as_deref().unwrap_or("")
                    ),
                })?;

        let db = state.db()?;

        let existing = ChargePoints::find()
            .filter(ChargePointColumn::ChargeBoxSerialNumber.eq(charge_box_serial_number.clone()))
            .filter(
                ChargePointColumn::ChargePointSerialNumber.eq(charge_point_serial_number.clone()),
            )
            .filter(ChargePointColumn::IsDeleted.eq(0_i16))
            .one(db)
            .await?;

        let now = Local::now().naive_local();
        let active = match existing {
            Some(model) => {
                let mut a: ActiveModel = model.into();
                a.charge_point_vendor = Set(Some(req.charge_point_vendor));
                a.charge_point_model = Set(Some(req.charge_point_model));
                a.firmware_version = Set(req.firmware_version.clone());
                a.iccid = Set(req.iccid.clone());
                a.imsi = Set(req.imsi.clone());
                a.meter_type = Set(req.meter_type.clone());
                a.meter_serial_number = Set(req.meter_serial_number.clone());
                a.update_time = Set(now);
                a
            }
            None => {
                return Err(AppError::OCPP_1_6_ERROR {
                    action: ACTION_BOOT_NOTIFICATION_CONFIRMATION.into(),
                    detail: format!(
                        "请求ID {}, charge_point_serial_number or charge_box_serial_number not exists",
                        msg.unique_id.as_deref().unwrap_or("")
                    ),
                });
            }
        };

        // 上报心跳到 DB（heartbeat 字段共用 update_time）
        active.update(db).await?;

        let confirmation = BootNotificationConfirmation::accepted(&Utc::now().to_rfc3339(), 30);
        Ok(confirmation)
    }
}
