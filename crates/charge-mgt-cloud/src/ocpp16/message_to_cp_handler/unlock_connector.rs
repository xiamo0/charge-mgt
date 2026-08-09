use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_UNLOCK_CONNECTOR_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::UnlockConnectorRequest;
use ocpp_1_6::confs::UnlockConnectorConfirmation;

impl Handler<UnlockConnectorConfirmation> for UnlockConnectorRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<UnlockConnectorConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_UNLOCK_CONNECTOR_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<UnlockConnectorConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_UNLOCK_CONNECTOR_CONFIRMATION).await
    }
}
