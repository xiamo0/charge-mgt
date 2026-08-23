//! `GET/PATCH /api/v1/charge-points/:pid/connectors[/...]` handler。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::auth::middleware::AuthContext;
use crate::auth::role;
use crate::error::AppError;
use crate::ocpp16::dto::charge_connector::{ChargeConnectorListQuery, UpdateChargeConnector};
use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::service::charge_connector as svc;
use crate::state::AppState;

/// `GET /api/v1/connectors` — 全局充电枪列表（不分桩）。
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargeConnectorListQuery>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::list(db, q).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `GET /api/v1/charge-points/:charge_point_id/connectors` — 嵌套端点，
/// 自动把 path 里的 `charge_point_id` 注入 list query。
pub async fn nested_list(
    Extension(state): Extension<Arc<AppState>>,
    Path(charge_point_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let q = ChargeConnectorListQuery {
        charge_point_id: Some(charge_point_id),
        ..Default::default()
    };
    if let Ok(db) = state.db() {
        let data = svc::list(db, q).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `GET /api/v1/charge-points/:charge_point_id/connectors/:connector_id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path((pid, cid)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(db) = state.db() {
        let data = svc::get(db, &pid, &cid).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}

/// `PATCH /api/v1/charge-points/:charge_point_id/connectors/:connector_id`
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Extension(ctx): Extension<AuthContext>,
    Path((pid, cid)): Path<(String, String)>,
    Json(req): Json<UpdateChargeConnector>,
) -> Result<impl IntoResponse, AppError> {
    role::require_write_access(&ctx)?;
    if let Ok(db) = state.db() {
        let data = svc::update(db, &pid, &cid, req).await?;
        Ok(Json(ApiResponse::ok(data)))
    } else {
        Err(AppError::ConfigNotInitialized("db".to_string()))
    }
}
