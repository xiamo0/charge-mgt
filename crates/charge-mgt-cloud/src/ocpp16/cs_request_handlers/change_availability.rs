use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::ChangeAvailabilityRequest;
use ocpp_1_6::confs::ChangeAvailabilityConfirmation;

impl Handler<ChangeAvailabilityConfirmation> for ChangeAvailabilityRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, HandlerError> {
        todo!()
    }
    #[cfg(feature = "cs_send_message_by_mq")]

    async fn mq_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, HandlerError> {
        todo!()
    }
}
