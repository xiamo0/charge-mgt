//! `GET/PATCH /api/v1/transactions[/...]` handler。
//!
//! 事务由 OCPP StartTransaction 创建，HTTP 入口**不**支持创建（无 POST）。

use std::sync::Arc;

use axum::extract::{Extension, Json, Path, Query};
use axum::response::IntoResponse;

use crate::dto::charge_transaction::{
    SettleTransaction, TransactionListQuery, UpdateTransaction,
};
use crate::dto::common::ApiResponse;
use crate::error::AppError;
use crate::service::charge_transaction as svc;
use crate::state::AppState;

/// `GET /api/v1/transactions`
pub async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<TransactionListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::list(&state.db, q).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/transactions/:id`
pub async fn get(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `GET /api/v1/transactions/by-transaction/:txn_id` — 按 OCPP 业务 ID 查。
pub async fn get_by_transaction_id(
    Extension(state): Extension<Arc<AppState>>,
    Path(txn_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::get_by_transaction_id(&state.db, &txn_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `PATCH /api/v1/transactions/:id`
pub async fn update(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTransaction>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::update(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/transactions/:id/settle` — 写入结算金额。
pub async fn settle(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<SettleTransaction>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::settle(&state.db, id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

/// `POST /api/v1/transactions/:id/refund` — 仅 `payment_status == Paid` 时允许。
pub async fn refund(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = svc::refund(&state.db, id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
