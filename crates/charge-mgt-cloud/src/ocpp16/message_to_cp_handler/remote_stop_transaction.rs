use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::RemoteStopTransactionRequest;
use ocpp_1_6::confs::RemoteStopTransactionConfirmation;

impl Handler<RemoteStopTransactionConfirmation> for RemoteStopTransactionRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<RemoteStopTransactionConfirmation, AppError> {
        todo!()
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<RemoteStopTransactionConfirmation, AppError> {
        todo!()
    }
}
