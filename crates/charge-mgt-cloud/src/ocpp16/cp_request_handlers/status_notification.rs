use crate::ocpp16::cp_request_handlers::Handler;
use ocpp_1_6::calls::StatusNotificationRequest;
use ocpp_1_6::confs::StatusNotificationConfirmation;

impl Handler<StatusNotificationConfirmation> for StatusNotificationRequest {
    async fn handel_detail(
        _state: &crate::state::AppState,
        _msg: &crate::ocpp16::envelope::CloudMessage,
    ) -> Result<StatusNotificationConfirmation, crate::ocpp16::error::HandlerError> {
        todo!()
    }
}
