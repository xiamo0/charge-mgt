use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::common::ApiResponse;
use crate::dto::smart_charge_profile::{CreateProfile, ProfileListQuery};
use crate::error::AppError;
use crate::service::profile as svc;
use crate::state::AppState;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ProfileListQuery>,
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

pub async fn nested_list(
    Extension(state): Extension<Arc<AppState>>,
    Path(charge_point_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list_by_charge_point(&state.db, &charge_point_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateProfile>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::create(&state.db, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn delete(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    svc::delete(&state.db, id).await?;
    Ok(Json(ApiResponse::ok("deleted".to_owned())))
}
