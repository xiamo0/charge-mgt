use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::charge_point::{
    ChargePointListQuery, CreateChargePoint, UpdateChargePoint,
};
use crate::dto::common::ApiResponse;
use crate::error::AppError;
use crate::service::charge_point as svc;
use crate::state::AppState;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargePointListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, &id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    svc::soft_delete(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok("deleted".to_owned())))
}

pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::restore(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
