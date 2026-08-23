//! `GET/POST/DELETE /api/v1/charging-profiles[/...]` handler。
//!
//! 注：策略下发 / 清除的 OCPP 协议交互由 OCPP 层负责，HTTP 仅写 DB 记录；
//! 真实下发需要在 caller 侧通过 `Kafka` / 内部消息触发。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::auth::middleware::AuthContext;
use crate::auth::role;
use crate::error::AppError;
use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::dto::smart_charge_profile::{CreateProfile, ProfileListQuery};
use crate::ocpp16::service::profile as svc;
use crate::state::AppState;

/// `GET /api/v1/charging-profiles`
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ProfileListQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::list(db, q).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `GET /api/v1/charging-profiles/:id`
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

/// `GET /api/v1/charge-points/:charge_point_id/charging-profiles` — 嵌套端点，
/// 返回该桩下所有策略（**不**分页）。
pub async fn nested_list(
    Extension(state): Extension<Arc<AppState>>,
    Path(charge_point_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::list_by_charge_point(db, &charge_point_id).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `POST /api/v1/charging-profiles`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Json(req): Json<CreateProfile>,
) -> Result<impl IntoResponse, AppError> {
    role::require_write_access(&ctx)?;
    if let Ok(db) = state.db() {
        let data = svc::create(db, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `DELETE /api/v1/charging-profiles/:id` — 物理删除。
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    role::require_delete_access(&ctx)?;
    if let Ok(db) = state.db() {
        svc::delete(db, id).await?;
        Ok(Json(ApiResponse::ok("deleted".to_owned())))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}
