use crate::error::AppError;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::ChangeAvailabilityRequest;
use ocpp_1_6::confs::ChangeAvailabilityConfirmation;
use ocpp_1_6::ACTION_CHANGE_AVAILABILITY_CONFIRMATION;

impl Handler<ChangeAvailabilityConfirmation> for ChangeAvailabilityRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_CHANGE_AVAILABILITY_CONFIRMATION).await
    }
    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_CHANGE_AVAILABILITY_CONFIRMATION).await
    }
}
