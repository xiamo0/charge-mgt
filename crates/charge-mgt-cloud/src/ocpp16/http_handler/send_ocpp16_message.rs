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
    #[cfg(feature = "message_by_http")]
    {
        let response = dispatch_http(&action, &state, &req).await?;
        return Ok(Json(response));
    }
    #[cfg(feature = "message_by_mq")]
    {
        let response = dispatch_mq(&action, &state, &req).await?;
        return Ok(Json(response));
    }
    #[allow(unreachable_code)]
    Err(AppError::ConfigNotInitialized(
        "neither message_by_http nor message_by_mq feature is enabled".into(),
    ))
}

#[cfg(feature = "message_by_http")]
async fn dispatch_http(
    action: &str,
    state: &Arc<AppState>,
    req: &CloudMessage,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    Ok(match action {
        ACTION_CANCEL_RESERVATION => ApiResponse::ok(CancelReservationRequest::handle_http(state, req).await?),
        ACTION_CHANGE_AVAILABILITY => ApiResponse::ok(ChangeAvailabilityRequest::handle_http(state, req).await?),
        ACTION_CHANGE_CONFIGURATION => ApiResponse::ok(ChangeConfigurationRequest::handle_http(state, req).await?),
        ACTION_CLEAR_CACHE => ApiResponse::ok(
            ClearCacheRequest::handle_http(state, req)
                .map_err(|e| AppError::BadRequest(e.to_string()))
                .await?,
        ),
        ACTION_CLEAR_CHARGING_PROFILE => ApiResponse::ok(ClearChargingProfileRequest::handle_http(state, req).await?),
        ACTION_DATA_TRANSFER => ApiResponse::ok(DataTransferRequest::handle_http(state, req).await?),
        ACTION_GET_COMPOSITE_SCHEDULE => ApiResponse::ok(GetCompositeScheduleRequest::handle_http(state, req).await?),
        ACTION_GET_CONFIGURATION => ApiResponse::ok(GetConfigurationRequest::handle_http(state, req).await?),
        ACTION_GET_DIAGNOSTICS => ApiResponse::ok(GetDiagnosticsRequest::handle_http(state, req).await?),
        ACTION_GET_LOCAL_LIST_VERSION => ApiResponse::ok(GetLocalListVersionRequest::handle_http(state, req).await?),
        ACTION_REMOTE_STOP_TRANSACTION => ApiResponse::ok(RemoteStopTransactionRequest::handle_http(state, req).await?),
        ACTION_REMOTE_START_TRANSACTION => ApiResponse::ok(RemoteStartTransactionRequest::handle_http(state, req).await?),
        ACTION_RESERVE_NOW => ApiResponse::ok(ReserveNowRequest::handle_http(state, req).await?),
        ACTION_SEND_LOCAL_LIST => ApiResponse::ok(SendLocalListRequest::handle_http(state, req).await?),
        ACTION_SET_CHARGING_PROFILE => ApiResponse::ok(SetChargingProfileRequest::handle_http(state, req).await?),
        ACTION_TRIGGER_MESSAGE => ApiResponse::ok(TriggerMessageRequest::handle_http(state, req).await?),
        ACTION_UNLOCK_CONNECTOR => ApiResponse::ok(UnlockConnectorRequest::handle_http(state, req).await?),
        ACTION_UPDATE_FIRMWARE => ApiResponse::ok(UpdateFirmwareRequest::handle_http(state, req).await?),
        _ => ApiResponse::ok(UnknownRequest::handle_http(state, req).await?),
    })
}

#[cfg(feature = "message_by_mq")]
async fn dispatch_mq(
    action: &str,
    state: &Arc<AppState>,
    req: &CloudMessage,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    Ok(match action {
        ACTION_CANCEL_RESERVATION => ApiResponse::ok(CancelReservationRequest::handle_mq(state, req).await?),
        ACTION_CHANGE_AVAILABILITY => ApiResponse::ok(ChangeAvailabilityRequest::handle_mq(state, req).await?),
        ACTION_CHANGE_CONFIGURATION => ApiResponse::ok(ChangeConfigurationRequest::handle_mq(state, req).await?),
        ACTION_CLEAR_CACHE => ApiResponse::ok(
            ClearCacheRequest::handle_mq(state, req)
                .map_err(|e| AppError::BadRequest(e.to_string()))
                .await?,
        ),
        ACTION_CLEAR_CHARGING_PROFILE => ApiResponse::ok(ClearChargingProfileRequest::handle_mq(state, req).await?),
        ACTION_DATA_TRANSFER => ApiResponse::ok(DataTransferRequest::handle_mq(state, req).await?),
        ACTION_GET_COMPOSITE_SCHEDULE => ApiResponse::ok(GetCompositeScheduleRequest::handle_mq(state, req).await?),
        ACTION_GET_CONFIGURATION => ApiResponse::ok(GetConfigurationRequest::handle_mq(state, req).await?),
        ACTION_GET_DIAGNOSTICS => ApiResponse::ok(GetDiagnosticsRequest::handle_mq(state, req).await?),
        ACTION_GET_LOCAL_LIST_VERSION => ApiResponse::ok(GetLocalListVersionRequest::handle_mq(state, req).await?),
        ACTION_REMOTE_STOP_TRANSACTION => ApiResponse::ok(RemoteStopTransactionRequest::handle_mq(state, req).await?),
        ACTION_REMOTE_START_TRANSACTION => ApiResponse::ok(RemoteStartTransactionRequest::handle_mq(state, req).await?),
        ACTION_RESERVE_NOW => ApiResponse::ok(ReserveNowRequest::handle_mq(state, req).await?),
        ACTION_SEND_LOCAL_LIST => ApiResponse::ok(SendLocalListRequest::handle_mq(state, req).await?),
        ACTION_SET_CHARGING_PROFILE => ApiResponse::ok(SetChargingProfileRequest::handle_mq(state, req).await?),
        ACTION_TRIGGER_MESSAGE => ApiResponse::ok(TriggerMessageRequest::handle_mq(state, req).await?),
        ACTION_UNLOCK_CONNECTOR => ApiResponse::ok(UnlockConnectorRequest::handle_mq(state, req).await?),
        ACTION_UPDATE_FIRMWARE => ApiResponse::ok(UpdateFirmwareRequest::handle_mq(state, req).await?),
        _ => ApiResponse::ok(UnknownRequest::handle_mq(state, req).await?),
    })
}
