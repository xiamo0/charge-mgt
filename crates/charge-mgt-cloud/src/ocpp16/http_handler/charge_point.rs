//! `GET/POST/PATCH/DELETE /api/v1/charge-points[/...]` handler。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::auth::middleware::AuthContext;
use crate::auth::role;
use crate::error::AppError;
use crate::ocpp16::dto::charge_point::{
    ChargePointListQuery, CreateChargePoint, UpdateChargePoint,
};
use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::service::charge_point as svc;
use crate::state::AppState;

/// `GET /api/v1/charge-points` — 列表分页查询。
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargePointListQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::list(db, q).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `GET /api/v1/charge-points/:charge_point_id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::get(db, &id).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `POST /api/v1/charge-points`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Json(req): Json<CreateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    role::require_write_access(&ctx)?;
    if let Ok(db) = state.db() {
        let data = svc::create(db, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `PATCH /api/v1/charge-points/:charge_point_id` — 部分更新。
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Path(id): Path<String>,
    Json(req): Json<UpdateChargePoint>,
) -> Result<impl IntoResponse, AppError> {
    role::require_write_access(&ctx)?;
    if let Ok(db) = state.db() {
        let data = svc::update(db, &id, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `DELETE /api/v1/charge-points/:charge_point_id` — **软删除**（置 `is_deleted=1`）。
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    role::require_delete_access(&ctx)?;
    if let Ok(db) = state.db() {
        svc::soft_delete(db, &id).await?;
        Ok(Json(ApiResponse::ok("deleted".to_owned())))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `POST /api/v1/charge-points/:charge_point_id/restore` — 恢复软删除。
pub async fn restore(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    role::require_write_access(&ctx)?;
    if let Ok(db) = state.db() {
        let data = svc::restore(db, &id).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}
