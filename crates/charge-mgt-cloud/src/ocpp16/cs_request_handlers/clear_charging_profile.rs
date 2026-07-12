use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeConfigurationRequest, ClearChargingProfileRequest,
};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation,
    ClearChargingProfileConfirmation,
};

impl Handler<ClearChargingProfileConfirmation> for ClearChargingProfileRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ClearChargingProfileConfirmation, HandlerError> {
        todo!()
    }
}
