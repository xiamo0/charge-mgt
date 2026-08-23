//! 鉴权 HTTP 端点
//!
//! - POST /api/v1/auth/login：登录拿 JWT
//! - GET  /api/v1/auth/me：当前登录账号信息（需 Bearer）

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::error::AppError;
use crate::state::AppState;

use super::middleware::AuthContext;
use super::service;

/// 公开路由：登录（无需 JWT）
pub fn public_router() -> Router {
    Router::new().route("/auth/login", post(login_handler))
}

/// 受保护路由：/auth/me（需 JWT，经过 require_auth 中间件）
pub fn protected_router() -> Router {
    Router::new().route("/auth/me", get(me_handler))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

async fn login_handler(
    Extension(state): Extension<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let resp = service::login(&state, &req.username, &req.password).await?;
    Ok((StatusCode::OK, Json(resp)))
}

async fn me_handler(
    Extension(state): Extension<AppState>,
    Extension(ctx): Extension<AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let op = service::get_operator(&state, ctx.operator_id).await?;
    Ok(Json(serde_json::json!({
        "id": op.id,
        "username": op.username,
        "role": op.role,
        "is_active": op.is_active,
    })))
}
