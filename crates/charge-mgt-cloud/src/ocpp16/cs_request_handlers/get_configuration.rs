use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeConfigurationRequest, GetCompositeScheduleRequest,
    GetConfigurationRequest,
};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation,
    GetCompositeScheduleConfirmation, GetConfigurationConfirmation,
};

impl Handler<GetConfigurationConfirmation> for GetConfigurationRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<GetConfigurationConfirmation, HandlerError> {
        todo!()
    }
}
