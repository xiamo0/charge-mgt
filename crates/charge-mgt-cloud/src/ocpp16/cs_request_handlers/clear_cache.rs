use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{CancelReservationRequest, ChangeConfigurationRequest, ClearCacheRequest};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation, ClearCacheConfirmation,
};

impl Handler<ClearCacheConfirmation> for ClearCacheRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<ClearCacheConfirmation, HandlerError> {
        todo!()
    }
}
