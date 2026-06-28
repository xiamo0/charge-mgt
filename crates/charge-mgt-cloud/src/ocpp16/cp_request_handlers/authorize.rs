use crate::ocpp16::cp_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::AuthorizeRequest;
use ocpp_1_6::confs::AuthorizeConfirmation;
impl Handler<AuthorizeConfirmation> for AuthorizeRequest {
    async fn handel_detail(
        _: &AppState,
        _: &CloudMessage,
    ) -> Result<AuthorizeConfirmation, HandlerError> {
        todo!()
    }
}
