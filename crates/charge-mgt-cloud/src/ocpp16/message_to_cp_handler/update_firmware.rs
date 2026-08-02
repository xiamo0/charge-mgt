use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::UpdateFirmwareRequest;
use ocpp_1_6::confs::UpdateFirmwareConfirmation;

impl Handler<UpdateFirmwareConfirmation> for UpdateFirmwareRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<UpdateFirmwareConfirmation, AppError> {
        todo!()
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<UpdateFirmwareConfirmation, AppError> {
        todo!()
    }
}
