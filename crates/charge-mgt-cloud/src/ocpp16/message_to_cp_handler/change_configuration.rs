use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_CHANGE_CONFIGURATION_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::ChangeConfigurationRequest;
use ocpp_1_6::confs::ChangeConfigurationConfirmation;

impl Handler<ChangeConfigurationConfirmation> for ChangeConfigurationRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail_http(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeConfigurationConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_CHANGE_CONFIGURATION_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail_mq(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeConfigurationConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_CHANGE_CONFIGURATION_CONFIRMATION).await
    }
}
