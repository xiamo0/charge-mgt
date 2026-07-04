use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::common::ApiResponse;
use crate::dto::identity_info::{CreateIdentity, IdentityListQuery, UpdateIdentity};
use crate::error::AppError;
use crate::service::identity as svc;
use crate::state::AppState;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<IdentityListQuery>,
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

pub async fn get_by_tag(
    Extension(state): Extension<Arc<AppState>>,
    Path(tag_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get_by_tag(&state.db, &tag_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateIdentity>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateIdentity>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::to_blocked_status(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("blocked".to_owned())))
}

pub async fn activate(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::activate(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn block(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::to_blocked_status(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("blocked".to_owned())))
}
