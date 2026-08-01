//! 应用层统一错误类型与 axum 响应映射。
//!
//! service 层所有函数都返回 [`AppError`]；handler 层用 `Result<T, AppError>`
//! 形式传递，axum 通过下文的 [`IntoResponse`] 实现自动转换为 HTTP 响应。
//!
//! # HTTP 状态码映射
//!
//! | 错误变体        | HTTP 状态 | 业务 code |
//! | --------------- | --------- | --------- |
//! | [`AppError::NotFound`]   | 404 NOT_FOUND       | 404 |
//! | [`AppError::BadRequest`] | 400 BAD_REQUEST     | 400 |
//! | [`AppError::Conflict`]   | 409 CONFLICT        | 409 |
//! | [`AppError::Internal`]   | 500 INTERNAL        | 500 |
//! | [`AppError::Db`]         | 500 INTERNAL        | 500 |
//! | [`AppError::Json`]       | 400 BAD_REQUEST     | 400 |
//!
//! 所有错误统一包成 [`ApiResponse::<()>::error`] 结构，与成功响应保持
//! `code` / `message` 字段一致，仅 `data` 缺失。

use axum::Json;
use axum::response::{IntoResponse, Response};
use sea_orm::{ColIdx, DbErr};

use crate::ocpp16::dto::common::ApiResponse;
use crate::ocpp16::error::HandlerError;

/// 应用层统一错误类型。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0} config not initialized")]
    ConfigNotInitialized(String),
    /// 资源不存在（如主键查询无果）；HTTP 404。
    #[error("not found: {0}")]
    NotFound(String),

    /// 请求语义错误（如业务前置条件不满足、参数范围违规）；HTTP 400。
    #[error("bad request: {0}")]
    BadRequest(String),

    /// 资源冲突（如 UNIQUE 索引重复）；HTTP 409。
    #[error("conflict: {0}")]
    Conflict(String),

    /// 内部逻辑错误（非 DB / 非 JSON 派生的运行时错误）；HTTP 500。
    #[error("internal error: {0}")]
    Internal(String),

    /// sea-orm DB 错误（含 UNIQUE 违反、外键失败等）；HTTP 500。
    #[error("db error: {0}")]
    Db(#[from] DbErr),

    /// serde_json 反/序列化错误；HTTP 400。
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("json error: {0}")]
    Handler(#[from] HandlerError),
}

impl AppError {
    /// 便捷构造 [`AppError::NotFound`]。
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// 便捷构造 [`AppError::BadRequest`]。
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    /// 便捷构造 [`AppError::Conflict`]。
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            Self::ConfigNotInitialized(msg) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                500,
                msg.clone(),
            ),
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
            Self::Handler(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                500,
                e.to_string(),
            ),
        };
        let body = Json(ApiResponse::<()>::error(code, message));
        (status, body).into_response()
    }
}
