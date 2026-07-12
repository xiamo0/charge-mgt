use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::ChangeAvailabilityRequest;
use ocpp_1_6::confs::ChangeAvailabilityConfirmation;

impl Handler<ChangeAvailabilityConfirmation> for ChangeAvailabilityRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ChangeAvailabilityConfirmation, HandlerError> {
        todo!()
    }
}
