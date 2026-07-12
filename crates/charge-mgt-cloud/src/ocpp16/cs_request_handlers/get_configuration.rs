use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::GetConfigurationRequest;
use ocpp_1_6::confs::GetConfigurationConfirmation;

impl Handler<GetConfigurationConfirmation> for GetConfigurationRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetConfigurationConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetConfigurationConfirmation, HandlerError> {
        todo!()
    }
}
