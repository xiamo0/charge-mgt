use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::SetChargingProfileRequest;
use ocpp_1_6::confs::SetChargingProfileConfirmation;

impl Handler<SetChargingProfileConfirmation> for SetChargingProfileRequest {
    #[cfg(feature = "send_message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<SetChargingProfileConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "send_message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<SetChargingProfileConfirmation, HandlerError> {
        todo!()
    }
}
