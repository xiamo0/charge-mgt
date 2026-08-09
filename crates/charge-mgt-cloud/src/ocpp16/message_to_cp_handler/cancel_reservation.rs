use crate::error::AppError;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::ACTION_CANCEL_RESERVATION_CONFIRMATION;
use ocpp_1_6::calls::CancelReservationRequest;
use ocpp_1_6::confs::CancelReservationConfirmation;

impl Handler<CancelReservationConfirmation> for CancelReservationRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_CANCEL_RESERVATION_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_CANCEL_RESERVATION_CONFIRMATION).await
    }
}
