use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{CancelReservationRequest, ChangeConfigurationRequest, GetCompositeScheduleRequest, GetConfigurationRequest, GetDiagnosticsRequest, GetLocalListVersionRequest, RemoteStartTransactionRequest, RemoteStopTransactionRequest};
use ocpp_1_6::confs::{CancelReservationConfirmation, ChangeConfigurationConfirmation, GetCompositeScheduleConfirmation, GetConfigurationConfirmation, GetDiagnosticsConfirmation, GetLocalListVersionConfirmation, RemoteStartTransactionConfirmation, RemoteStopTransactionConfirmation};

impl Handler<RemoteStartTransactionConfirmation> for RemoteStartTransactionRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<RemoteStartTransactionConfirmation, HandlerError> {
        todo!()
    }
}
