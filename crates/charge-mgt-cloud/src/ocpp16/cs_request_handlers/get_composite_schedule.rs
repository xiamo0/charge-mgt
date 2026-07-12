use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeConfigurationRequest, GetCompositeScheduleRequest,
};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation,
    GetCompositeScheduleConfirmation,
};

impl Handler<GetCompositeScheduleConfirmation> for GetCompositeScheduleRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<GetCompositeScheduleConfirmation, HandlerError> {
        todo!()
    }
}
