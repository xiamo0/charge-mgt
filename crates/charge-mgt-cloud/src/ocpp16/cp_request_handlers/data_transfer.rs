use crate::ocpp16::cp_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::DataTransferRequest;
use ocpp_1_6::confs::DataTransferConfirmation;
impl Handler<DataTransferConfirmation> for DataTransferRequest {
    async fn handel_detail(
        _: &AppState,
        _: &CloudMessage,
    ) -> Result<DataTransferConfirmation, HandlerError> {
        todo!()
    }
}
