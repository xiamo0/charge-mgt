use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use ocpp_1_6::calls::StartTransactionRequest;
use ocpp_1_6::confs::StartTransactionConfirmation;

impl Handler<StartTransactionConfirmation> for StartTransactionRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        _msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<StartTransactionConfirmation, AppError> {
        todo!()
    }
}
