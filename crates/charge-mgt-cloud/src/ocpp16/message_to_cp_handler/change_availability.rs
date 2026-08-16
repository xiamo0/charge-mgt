use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::ACTION_CHANGE_AVAILABILITY_CONFIRMATION;
use ocpp_1_6::calls::ChangeAvailabilityRequest;
use ocpp_1_6::confs::ChangeAvailabilityConfirmation;

impl Handler<ChangeAvailabilityConfirmation> for ChangeAvailabilityRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail_http(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_CHANGE_AVAILABILITY_CONFIRMATION).await
    }
    #[cfg(feature = "message_by_mq")]
    async fn handle_detail_mq(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_CHANGE_AVAILABILITY_CONFIRMATION).await
    }
}
