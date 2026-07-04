//! `GET/POST/PATCH/DELETE /api/v1/charge-points[/...]` handler。

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

/// `GET /api/v1/charge-points` — 列表分页查询。
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargePointListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/charge-points/:charge_point_id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/charge-points`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `PATCH /api/v1/charge-points/:charge_point_id` — 部分更新。
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, &id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `DELETE /api/v1/charge-points/:charge_point_id` — **软删除**（置 `is_deleted=1`）。
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    svc::soft_delete(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok("deleted".to_owned())))
}

/// `POST /api/v1/charge-points/:charge_point_id/restore` — 恢复软删除。
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::restore(&state.db, &id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
