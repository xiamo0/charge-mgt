use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::TriggerMessageRequest;
use ocpp_1_6::confs::TriggerMessageConfirmation;

impl Handler<TriggerMessageConfirmation> for TriggerMessageRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<TriggerMessageConfirmation, AppError> {
        todo!()
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<TriggerMessageConfirmation, AppError> {
        todo!()
    }
}
