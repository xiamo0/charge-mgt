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
mod remote_start_transaction;
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
        let r = Self::handel_detail(state, msg).await?;

        Ok(serde_json::to_value(&r)?)
    }
    async fn handel_detail(
        state: &crate::state::AppState,
        msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<T, HandlerError> {
        #[cfg(feature = "cs_send_message_by_http")]
        {
            return Self::http_handler(state, msg).await;
        }
        #[cfg(feature = "cs_send_message_by_mq")]
        {
            return Self::mq_handler(state, msg).await;
        }
        // 兜底处理：如果两个 feature 都没有启用，编译时直接报错
        #[cfg(not(any(feature = "cs_send_message_by_http", feature = "cs_send_message_by_mq")))]
        {
            Err(HandlerError::ConfigError(
                "No message sending backend configured. Enable 'cs_send_message_by_http' or 'cs_send_message_by_mq' feature.".into()
            ))
        }
    }

    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(state: &AppState, msg: &CloudMessage) -> Result<T, HandlerError>;
    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(state: &AppState, msg: &CloudMessage) -> Result<T, HandlerError>;
}
pub struct UnkonwnRequest;
impl Handler<String> for UnkonwnRequest {
    async fn http_handler(_state: &AppState, msg: &CloudMessage) -> Result<String, HandlerError> {
        let action = msg.action.as_str();
        Err(HandlerError::NotSupported(format!(
            "action '{action}' not implemented"
        )))
    }

    async fn mq_handler(_state: &AppState, msg: &CloudMessage) -> Result<String, HandlerError> {
        let action = msg.action.as_str();
        Err(HandlerError::NotSupported(format!(
            "action '{action}' not implemented"
        )))
    }
}
