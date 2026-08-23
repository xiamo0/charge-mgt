//! 内网 API（仅 gateway 调用，校验 Bearer token + Basic Auth 密码验证）

pub mod auth;
pub mod middleware;

use axum::Router;

/// 内网 API 路由：仅由中间件保护，必须挂载在 `require_internal_token` 之后
pub fn router() -> Router {
    Router::new().merge(auth::router())
}
