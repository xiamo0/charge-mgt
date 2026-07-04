//! `GET/PATCH /api/v1/charge-points/:pid/connectors[/...]` handler。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::charge_connector::{ChargeConnectorListQuery, UpdateChargeConnector};
use crate::dto::common::ApiResponse;
use crate::error::AppError;
use crate::service::charge_connector as svc;
use crate::state::AppState;

/// `GET /api/v1/connectors` — 全局充电枪列表（不分桩）。
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargeConnectorListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
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
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/charge-points/:charge_point_id/connectors/:connector_id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path((pid, cid)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, &pid, &cid).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `PATCH /api/v1/charge-points/:charge_point_id/connectors/:connector_id`
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path((pid, cid)): Path<(String, String)>,
    Json(req): Json<UpdateChargeConnector>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, &pid, &cid, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
