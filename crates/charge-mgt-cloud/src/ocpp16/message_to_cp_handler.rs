use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use serde::Serialize;

// 云平台向充电桩发送请求
pub mod cancel_reservation;
pub mod change_availability;
pub mod change_configuration;
pub mod clear_cache;
pub mod clear_charging_profile;
pub mod data_transfer;
pub mod get_composite_schedule;
pub mod get_configuration;
pub mod get_diagnostics;
pub mod get_local_list_version;
pub mod remote_start_transaction;
pub mod remote_stop_transaction;
pub mod reserve_now;
pub mod send_local_list;
pub mod set_charging_profile;
pub mod trigger_message;
pub mod unlock_connector;
pub mod update_firmware;

pub trait Handler<T: Serialize> {
    async fn handle(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<serde_json::Value, HandlerError> {
        let r = Self::handle_detail(state, msg).await?;

        Ok(serde_json::to_value(&r)?)
    }

    async fn handle_detail(state: &AppState, msg: &CloudMessage) -> Result<T, HandlerError>;
}
pub struct UnknownRequest;
impl Handler<String> for UnknownRequest {
    #[cfg(feature = "send_message_by_http")]
    async fn handle_detail(_state: &AppState, msg: &CloudMessage) -> Result<String, HandlerError> {
        let action = msg.action.as_str();
        Err(HandlerError::NotSupported(format!(
            "action '{action}' not implemented"
        )))
    }

    #[cfg(feature = "send_message_by_mq")]
    async fn handle_detail(_state: &AppState, msg: &CloudMessage) -> Result<String, HandlerError> {
        let action = msg.action.as_str();
        Err(HandlerError::NotSupported(format!(
            "action '{action}' not implemented"
        )))
    }
}
