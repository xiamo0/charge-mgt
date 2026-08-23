//! 内网 API：桩身份验证
//!
//! POST /internal/auth/verify
//! Body: { "charge_point_id": "...", "password": "..." }
//! Response: { "valid": true|false }
//!
//! **不区分失败原因**：不存在/已删/未设密码/密码错 一律返 `{"valid": false}`，
//! 防止 charge_point_id 枚举攻击。

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::ocpp16::service::charge_point::verify_password;
use crate::state::AppState;

pub fn router() -> Router {
    Router::new().route("/auth/verify", post(verify_handler))
}

#[derive(Debug, Deserialize)]
pub struct VerifyAuthRequest {
    pub charge_point_id: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyAuthResponse {
    pub valid: bool,
}

async fn verify_handler(
    Extension(state): Extension<AppState>,
    Json(req): Json<VerifyAuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let valid = verify_password(state.db()?, &req.charge_point_id, &req.password).await?;
    // 注意：密码错也返 200（business 失败），用 valid=false 区分。
    // 网络/系统错才返 5xx（让 gateway fail-closed）
    Ok((StatusCode::OK, Json(VerifyAuthResponse { valid })))
}
