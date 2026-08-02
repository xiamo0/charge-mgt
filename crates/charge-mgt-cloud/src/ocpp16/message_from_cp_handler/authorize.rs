use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::AuthorizeRequest;
use ocpp_1_6::confs::AuthorizeConfirmation;

impl Handler<AuthorizeConfirmation> for AuthorizeRequest {
    async fn handel_detail(
        _: &AppState,
        _: &CloudMessage,
    ) -> Result<AuthorizeConfirmation, AppError> {
        todo!()
    }
}
