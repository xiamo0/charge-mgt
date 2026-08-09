use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_UPDATE_FIRMWARE_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::UpdateFirmwareRequest;
use ocpp_1_6::confs::UpdateFirmwareConfirmation;

impl Handler<UpdateFirmwareConfirmation> for UpdateFirmwareRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<UpdateFirmwareConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_UPDATE_FIRMWARE_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<UpdateFirmwareConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_UPDATE_FIRMWARE_CONFIRMATION).await
    }
}
