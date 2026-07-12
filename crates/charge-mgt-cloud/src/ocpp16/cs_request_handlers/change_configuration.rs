use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{CancelReservationRequest, ChangeConfigurationRequest};
use ocpp_1_6::confs::{CancelReservationConfirmation, ChangeConfigurationConfirmation};

impl Handler<ChangeConfigurationConfirmation> for ChangeConfigurationRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(state: &AppState, msg: &CloudMessage) -> Result<ChangeConfigurationConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeConfigurationConfirmation, HandlerError> {
        todo!()
    }
}

