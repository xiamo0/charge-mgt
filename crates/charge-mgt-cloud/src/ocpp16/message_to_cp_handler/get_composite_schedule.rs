use crate::ocpp16::envelope::CloudMessage;
use ocpp_1_6::ACTION_GET_COMPOSITE_SCHEDULE_CONFIRMATION;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
#[cfg(feature = "message_by_mq")]
use crate::ocpp16::message_to_cp_handler::dispatch_mq_call;
#[cfg(feature = "message_by_http")]
use crate::ocpp16::message_to_cp_handler::dispatch_http_call;
use crate::state::AppState;
use ocpp_1_6::calls::GetCompositeScheduleRequest;
use ocpp_1_6::confs::GetCompositeScheduleConfirmation;

impl Handler<GetCompositeScheduleConfirmation> for GetCompositeScheduleRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<GetCompositeScheduleConfirmation, AppError> {
        dispatch_http_call(state, msg, ACTION_GET_COMPOSITE_SCHEDULE_CONFIRMATION).await
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<GetCompositeScheduleConfirmation, AppError> {
        dispatch_mq_call(state, msg, ACTION_GET_COMPOSITE_SCHEDULE_CONFIRMATION).await
    }
}
