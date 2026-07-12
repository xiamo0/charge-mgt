use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::SetChargingProfileRequest;
use ocpp_1_6::confs::SetChargingProfileConfirmation;

impl Handler<SetChargingProfileConfirmation> for SetChargingProfileRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<SetChargingProfileConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<SetChargingProfileConfirmation, HandlerError> {
        todo!()
    }
}
