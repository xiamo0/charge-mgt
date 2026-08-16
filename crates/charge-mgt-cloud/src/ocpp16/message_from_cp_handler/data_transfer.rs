use crate::error::AppError;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use charge_mgt_common::ocpp16::CloudMessage;
use ocpp_1_6::calls::DataTransferRequest;
use ocpp_1_6::confs::DataTransferConfirmation;

impl Handler<DataTransferConfirmation> for DataTransferRequest {
    async fn handel_detail(
        _: &AppState,
        _: &CloudMessage,
    ) -> Result<DataTransferConfirmation, AppError> {
        todo!()
    }
}
