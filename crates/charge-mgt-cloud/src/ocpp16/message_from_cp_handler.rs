// 云平台处理充电桩的请求
pub mod authorize;
pub mod boot_notification;
pub mod data_transfer;
pub mod heartbeat;
pub mod meter_values;
pub mod start_transaction;
pub mod status_notification;
pub mod stop_transaction;

use crate::ocpp16::error::HandlerError;
use serde::Serialize;
pub trait Handler<T: Serialize> {
    async fn handle(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<serde_json::Value, crate::ocpp16::error::HandlerError> {
        let r = Self::handel_detail(state, msg).await?;

        Ok(serde_json::to_value(&r)?)
    }
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<T, crate::ocpp16::error::HandlerError>;
}
pub struct UnknownRequest;
impl Handler<String> for UnknownRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<std::string::String, crate::ocpp16::error::HandlerError> {
        let action = msg.action.as_str();
        Err(HandlerError::NotSupported(format!(
            "action '{action}' not implemented"
        )))
    }
}
