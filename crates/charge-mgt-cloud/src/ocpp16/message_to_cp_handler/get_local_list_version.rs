use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::GetLocalListVersionRequest;
use ocpp_1_6::confs::GetLocalListVersionConfirmation;

impl Handler<GetLocalListVersionConfirmation> for GetLocalListVersionRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetLocalListVersionConfirmation, AppError> {
        todo!()
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetLocalListVersionConfirmation, AppError> {
        todo!()
    }
}
