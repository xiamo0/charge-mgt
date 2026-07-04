//! `GET/POST/PATCH/DELETE /api/v1/identities[/...]` handler。
//!
//! DELETE 行为说明：把 `status` 置为 `Blocked`（**不**物理删除），
//! 保留审计轨迹；与 `charge_point` 的 `is_deleted` 软删除是两种不同的
//! 删除语义。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::common::ApiResponse;
use crate::dto::identity_info::{CreateIdentity, IdentityListQuery, UpdateIdentity};
use crate::error::AppError;
use crate::service::identity as svc;
use crate::state::AppState;

/// `GET /api/v1/identities`
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<IdentityListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/identities/:id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/identities/by-tag/:tag_id` — 按 UNIQUE 列查。
pub async fn get_by_tag(
    Extension(state): Extension<Arc<AppState>>,
    Path(tag_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get_by_tag(&state.db, &tag_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/identities`
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateIdentity>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `PATCH /api/v1/identities/:id`
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateIdentity>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `DELETE /api/v1/identities/:id` — 业务上等价于 `block`。
///
/// **不**物理删除；仅置 `status = Blocked`。
pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::to_blocked_status(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("blocked".to_owned())))
}

/// `POST /api/v1/identities/:id/activate` — 把 `Blocked` 改回 `Accepted`。
///
/// **前置**：`Expired` 标签不能直接 activate（必须先续期）。
pub async fn activate(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::activate(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/identities/:id/block` — 显式挂失（与 DELETE 等价）。
pub async fn block(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::to_blocked_status(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("blocked".to_owned())))
}
