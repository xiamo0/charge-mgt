use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::charge_reservation::{
    CancelReservation, CreateReservation, ReservationListQuery, UpdateReservation,
};
use crate::dto::common::ApiResponse;
use crate::error::AppError;
use crate::service::reservation as svc;
use crate::state::AppState;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ReservationListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateReservation>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateReservation>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn cancel(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<CancelReservation>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::cancel(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
