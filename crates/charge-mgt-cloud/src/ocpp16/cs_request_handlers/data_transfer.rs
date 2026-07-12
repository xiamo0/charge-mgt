use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{CancelReservationRequest, ChangeConfigurationRequest, DataTransferRequest};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation, DataTransferConfirmation,
};

impl Handler<DataTransferConfirmation> for DataTransferRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<DataTransferConfirmation, HandlerError> {
        todo!()
    }
}
