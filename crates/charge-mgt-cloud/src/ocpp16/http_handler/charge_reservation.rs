//! `GET/POST/PATCH /api/v1/reservations[/...]` handler。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::error::AppError;
use crate::ocpp16::dto::charge_reservation::{
    CancelReservation, CreateReservation, ReservationListQuery, UpdateReservation,
};
use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::service::reservation as svc;
use crate::state::AppState;

/// `GET /api/v1/reservations`
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ReservationListQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::list(db, q).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `GET /api/v1/reservations/:id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::get(db, id).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `POST /api/v1/reservations`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateReservation>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::create(db, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `PATCH /api/v1/reservations/:id` — 仅 `status == Pending` 时允许。
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateReservation>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::update(db, id, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `POST /api/v1/reservations/:id/cancel` — 仅 `status == Pending` 时允许。
pub async fn cancel(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<CancelReservation>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::cancel(db, id, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}
