use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::charge_connector::{ChargeConnectorListQuery, UpdateChargeConnector};
use crate::dto::common::ApiResponse;
use crate::error::AppError;
use crate::service::charge_connector as svc;
use crate::state::AppState;

pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<ChargeConnectorListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

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

pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path((pid, cid)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, &pid, &cid).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path((pid, cid)): Path<(String, String)>,
    Json(req): Json<UpdateChargeConnector>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, &pid, &cid, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
