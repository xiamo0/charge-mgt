use crate::ocpp16::envelope::CloudMessage;

use crate::error::AppError;
use crate::ocpp16::message_to_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::GetConfigurationRequest;
use ocpp_1_6::confs::GetConfigurationConfirmation;

impl Handler<GetConfigurationConfirmation> for GetConfigurationRequest {
    #[cfg(feature = "message_by_http")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetConfigurationConfirmation, AppError> {
        todo!()
    }

    #[cfg(feature = "message_by_mq")]
    async fn handle_detail(
        _state: &AppState,
        _msg: &CloudMessage,
    ) -> Result<GetConfigurationConfirmation, AppError> {
        todo!()
    }
}
