use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::CancelReservationRequest;
use ocpp_1_6::confs::CancelReservationConfirmation;

impl Handler<CancelReservationConfirmation> for CancelReservationRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<CancelReservationConfirmation, HandlerError> {
        #[cfg(feature = "cs_send_message_by_http")]
        {
            return http_handler(state, msg).await;
        }
        #[cfg(feature = "cs_send_message_by_mq")]
        {
            return mq_handler(state, msg).await;
        }
        // 兜底处理：如果两个 feature 都没有启用，编译时直接报错
        #[cfg(not(any(feature = "cs_send_message_by_http", feature = "cs_send_message_by_mq")))]
        {
            Err(HandlerError::ConfigError(
                "No message sending backend configured. Enable 'cs_send_message_by_http' or 'cs_send_message_by_mq' feature.".into()
            ))
        }
    }
}

#[cfg(feature = "cs_send_message_by_http")]
async fn http_handler(
    state: &AppState,
    msg: &CloudMessage,
) -> Result<CancelReservationConfirmation, HandlerError> {
    todo!()
}
#[cfg(feature = "cs_send_message_by_mq")]
async fn mq_handler(
    state: &AppState,
    msg: &CloudMessage,
) -> Result<CancelReservationConfirmation, HandlerError> {
    todo!()
}
