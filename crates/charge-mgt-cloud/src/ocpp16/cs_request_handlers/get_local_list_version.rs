use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeConfigurationRequest, GetCompositeScheduleRequest,
    GetConfigurationRequest, GetDiagnosticsRequest, GetLocalListVersionRequest,
};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation,
    GetCompositeScheduleConfirmation, GetConfigurationConfirmation, GetDiagnosticsConfirmation,
    GetLocalListVersionConfirmation,
};

impl Handler<GetLocalListVersionConfirmation> for GetLocalListVersionRequest {
    #[cfg(feature = "cs_send_message_by_http")]
    async fn http_handler(state: &AppState, msg: &CloudMessage) -> Result<GetLocalListVersionConfirmation, HandlerError> {
        todo!()
    }

    #[cfg(feature = "cs_send_message_by_mq")]
    async fn mq_handler(state: &AppState, msg: &CloudMessage) -> Result<GetLocalListVersionConfirmation, HandlerError> {
        todo!()
    }
}
