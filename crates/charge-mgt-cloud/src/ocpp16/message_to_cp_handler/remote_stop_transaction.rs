use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_REMOTE_STOP_TRANSACTION_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::RemoteStopTransactionRequest;
use ocpp_1_6::confs::RemoteStopTransactionConfirmation;

impl Handler<RemoteStopTransactionConfirmation> for RemoteStopTransactionRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<RemoteStopTransactionConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_REMOTE_STOP_TRANSACTION_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<RemoteStopTransactionConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_REMOTE_STOP_TRANSACTION_CONFIRMATION).await
    }
}
