// 云平台处理充电桩的请求
pub mod authorize;
pub mod boot_notification;
pub mod data_transfer;
pub mod diagnostics_status_notification;
pub mod firmware_status_notification;
pub mod heartbeat;
pub mod meter_values;
pub mod start_transaction;
pub mod status_notification;
pub mod stop_transaction;

use crate::error::AppError;
use serde::Serialize;

pub trait Handler<T: Serialize> {
    async fn handle(
        state: &crate::state::AppState,
        msg: &charge_mgt_common::ocpp16::CloudMessage,
    ) -> Result<serde_json::Value, AppError> {
        let r = Self::handel_detail(state, msg).await?;

        Ok(serde_json::to_value(&r)?)
    }
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &charge_mgt_common::ocpp16::CloudMessage,
    ) -> Result<T, AppError>;
}
pub struct UnknownRequest;
impl Handler<String> for UnknownRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        msg: &charge_mgt_common::ocpp16::CloudMessage,
    ) -> Result<std::string::String, AppError> {
        let action = msg.action.as_deref().unwrap_or("");
        Err(AppError::OCPP_1_6_ERROR {
            action: action.into(),
            detail: "action not implemented".into(),
        })
    }
}
