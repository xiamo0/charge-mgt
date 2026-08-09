use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_REMOTE_START_TRANSACTION_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::RemoteStartTransactionRequest;
use ocpp_1_6::confs::RemoteStartTransactionConfirmation;

impl Handler<RemoteStartTransactionConfirmation> for RemoteStartTransactionRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail_http(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<RemoteStartTransactionConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_REMOTE_START_TRANSACTION_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail_mq(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<RemoteStartTransactionConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_REMOTE_START_TRANSACTION_CONFIRMATION).await
    }
}
