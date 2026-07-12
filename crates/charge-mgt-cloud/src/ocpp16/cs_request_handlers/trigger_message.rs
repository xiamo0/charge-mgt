use crate::ocpp16::cs_request_handlers::Handler;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::error::HandlerError;
use crate::state::AppState;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeConfigurationRequest, GetCompositeScheduleRequest,
    GetConfigurationRequest, GetDiagnosticsRequest, GetLocalListVersionRequest,
    RemoteStopTransactionRequest, ReserveNowRequest, SendLocalListRequest,
    SetChargingProfileRequest, TriggerMessageRequest,
};
use ocpp_1_6::confs::{
    CancelReservationConfirmation, ChangeConfigurationConfirmation,
    GetCompositeScheduleConfirmation, GetConfigurationConfirmation, GetDiagnosticsConfirmation,
    GetLocalListVersionConfirmation, RemoteStopTransactionConfirmation, ReserveNowConfirmation,
    SendLocalListConfirmation, SetChargingProfileConfirmation, TriggerMessageConfirmation,
};

impl Handler<TriggerMessageConfirmation> for TriggerMessageRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<TriggerMessageConfirmation, HandlerError> {
        todo!()
    }
}
