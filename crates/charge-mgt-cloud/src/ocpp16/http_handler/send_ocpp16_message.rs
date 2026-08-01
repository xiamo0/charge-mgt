use crate::error::AppError;
use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_to_cp_handler::{Handler, UnknownRequest};
use crate::state::AppState;
use axum::Json;
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use futures::TryFutureExt;
use ocpp_1_6::calls::{
    CancelReservationRequest, ChangeAvailabilityRequest, ChangeConfigurationRequest,
    ClearCacheRequest, ClearChargingProfileRequest, DataTransferRequest,
    GetCompositeScheduleRequest, GetConfigurationRequest, GetDiagnosticsRequest,
    GetLocalListVersionRequest, RemoteStartTransactionRequest, RemoteStopTransactionRequest,
    ReserveNowRequest, SendLocalListRequest, SetChargingProfileRequest, TriggerMessageRequest,
    UnlockConnectorRequest, UpdateFirmwareRequest,
};
use ocpp_1_6::protocol::{
    ACTION_CANCEL_RESERVATION, ACTION_CHANGE_AVAILABILITY, ACTION_CHANGE_CONFIGURATION,
    ACTION_CLEAR_CACHE, ACTION_CLEAR_CHARGING_PROFILE, ACTION_DATA_TRANSFER,
    ACTION_GET_COMPOSITE_SCHEDULE, ACTION_GET_CONFIGURATION, ACTION_GET_DIAGNOSTICS,
    ACTION_GET_LOCAL_LIST_VERSION, ACTION_REMOTE_START_TRANSACTION, ACTION_REMOTE_STOP_TRANSACTION,
    ACTION_RESERVE_NOW, ACTION_SEND_LOCAL_LIST, ACTION_SET_CHARGING_PROFILE,
    ACTION_TRIGGER_MESSAGE, ACTION_UNLOCK_CONNECTOR, ACTION_UPDATE_FIRMWARE,
};
use std::sync::Arc;

pub async fn send(
    Extension(state): Extension<Arc<AppState>>,
    Path(action): Path<String>,
    Json(req): Json<CloudMessage>,
) -> Result<impl IntoResponse, AppError> {
    let response = match action.as_str() {
        ACTION_CANCEL_RESERVATION => {
            let data = CancelReservationRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_CHANGE_AVAILABILITY => {
            // Handle call result action
            let data = ChangeAvailabilityRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_CHANGE_CONFIGURATION => {
            let data = ChangeConfigurationRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_CLEAR_CACHE => {
            let data = ClearCacheRequest::handle(&state, &req)
                .map_err(|e| AppError::BadRequest(e.to_string()))
                .await?;
            ApiResponse::ok(data)
        }
        ACTION_CLEAR_CHARGING_PROFILE => {
            let data = ClearChargingProfileRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_DATA_TRANSFER => {
            let data = DataTransferRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_GET_COMPOSITE_SCHEDULE => {
            let data = GetCompositeScheduleRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_GET_CONFIGURATION => {
            let data = GetConfigurationRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_GET_DIAGNOSTICS => {
            let data = GetDiagnosticsRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_GET_LOCAL_LIST_VERSION => {
            let data = GetLocalListVersionRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_REMOTE_STOP_TRANSACTION => {
            let data = RemoteStopTransactionRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_REMOTE_START_TRANSACTION => {
            let data = RemoteStartTransactionRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_RESERVE_NOW => {
            let data = ReserveNowRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_SEND_LOCAL_LIST => {
            let data = SendLocalListRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_SET_CHARGING_PROFILE => {
            let data = SetChargingProfileRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }

        ACTION_TRIGGER_MESSAGE => {
            let data = TriggerMessageRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_UNLOCK_CONNECTOR => {
            let data = UnlockConnectorRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        ACTION_UPDATE_FIRMWARE => {
            let data = UpdateFirmwareRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
        _ => {
            let data = UnknownRequest::handle(&state, &req).await?;
            ApiResponse::ok(data)
        }
    };

    Ok(Json(response))
}
