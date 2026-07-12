//! axum handler 层。
//!
//! 每个 handler 仅做 axum extractor 抽取（Path / Query / Json）+ 调对应
//! service 函数 + 包成 [`crate::dto::common::ApiResponse`]，**不**承载任何业务逻辑。
//!
//! 错误流转：service 返回 [`crate::error::AppError`]，通过 axum `IntoResponse`
//! 自动映射到 HTTP 状态码与 JSON 错误壳。

pub mod charge_connector;
pub mod charge_point;
pub mod charge_reservation;
pub mod charge_transaction;
pub mod identity;
pub mod profile;
pub mod send_ocpp16_message;
