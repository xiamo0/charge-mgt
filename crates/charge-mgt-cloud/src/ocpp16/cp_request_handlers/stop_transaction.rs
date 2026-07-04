use crate::ocpp16::cp_request_handlers::Handler;
use ocpp_1_6::calls::StopTransactionRequest;
use ocpp_1_6::confs::StopTransactionConfirmation;

impl Handler<StopTransactionConfirmation> for StopTransactionRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        _msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<StopTransactionConfirmation, crate::ocpp16::error::HandlerError> {
        todo!()
    }
}
