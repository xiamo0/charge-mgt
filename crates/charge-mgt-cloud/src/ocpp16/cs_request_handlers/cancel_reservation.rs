use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::CancelReservationRequest;
use ocpp_1_6::confs::CancelReservationConfirmation;

impl Handler<CancelReservationConfirmation> for CancelReservationRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, HandlerError> {
        todo!()
    }
}
