//! `GET/POST/DELETE /api/v1/charging-profiles[/...]` handler。
//!
//! 注：策略下发 / 清除的 OCPP 协议交互由 OCPP 层负责，HTTP 仅写 DB 记录；
//! 真实下发需要在 caller 侧通过 `Kafka` / 内部消息触发。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::common::ApiResponse;
use crate::dto::smart_charge_profile::{CreateProfile, ProfileListQuery};
use crate::error::AppError;
use crate::service::profile as svc;
use crate::state::AppState;

/// `GET /api/v1/charging-profiles`
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ProfileListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/charging-profiles/:id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/charge-points/:charge_point_id/charging-profiles` — 嵌套端点，
/// 返回该桩下所有策略（**不**分页）。
pub async fn nested_list(
    Extension(state): Extension<Arc<AppState>>,
    Path(charge_point_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list_by_charge_point(&state.db, &charge_point_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/charging-profiles`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateProfile>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `DELETE /api/v1/charging-profiles/:id` — 物理删除。
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::delete(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("deleted".to_owned())))
}
