use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::ReserveNowRequest;
use ocpp_1_6::confs::ReserveNowConfirmation;

impl Handler<ReserveNowConfirmation> for ReserveNowRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<ReserveNowConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<ReserveNowConfirmation, HandlerError> {
        todo!()
    }
}
