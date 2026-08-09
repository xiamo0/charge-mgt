use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_SEND_LOCAL_LIST_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::SendLocalListRequest;
use ocpp_1_6::confs::SendLocalListConfirmation;

impl Handler<SendLocalListConfirmation> for SendLocalListRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<SendLocalListConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_SEND_LOCAL_LIST_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<SendLocalListConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_SEND_LOCAL_LIST_CONFIRMATION).await
    }
}
