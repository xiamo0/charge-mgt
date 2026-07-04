use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DbErr;

use crate::dto::common::ApiResponse;

/// 应用层统一错误类型。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("db error: {0}")]
    Db(#[from] DbErr),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl AppError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            Self::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, 404, msg.clone()),
            Self::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, 400, msg.clone()),
            Self::Conflict(msg) => (axum::http::StatusCode::CONFLICT, 409, msg.clone()),
            Self::Internal(msg) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                500,
                msg.clone(),
            ),
            Self::Db(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                500,
                e.to_string(),
            ),
            Self::Json(e) => (axum::http::StatusCode::BAD_REQUEST, 400, e.to_string()),
        };
        let body = Json(ApiResponse::<()>::error(code, message));
        (status, body).into_response()
    }
}
