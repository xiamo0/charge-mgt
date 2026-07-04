use crate::ocpp16::cp_request_handlers::Handler;
use ocpp_1_6::calls::MeterValuesRequest;
use ocpp_1_6::confs::MeterValuesConfirmation;

impl Handler<MeterValuesConfirmation> for MeterValuesRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        _msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<MeterValuesConfirmation, crate::ocpp16::error::HandlerError> {
        todo!()
    }
}
