//! HTTP 鉴权中间件：解析 Bearer JWT，注入 AuthContext
//!
//! 白名单路由（/health、/、/api/v1/auth/login）不经过本中间件。

use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::ocpp16::entity::operator::Role;

/// 注入到请求 extension 的认证上下文
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub operator_id: i64,
    pub username: String,
    pub role: Role,
}

/// 验证 Bearer JWT；失败返 401
pub fn require_auth(
    secret: String,
) -> impl Fn(Request<axum::body::Body>, Next<axum::body::Body>) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
> + Clone {
    move |req: Request<axum::body::Body>, next: Next<axum::body::Body>| {
        let secret = secret.clone();
        Box::pin(async move {
            let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "));

            let Some(token) = token else {
                return Err(StatusCode::UNAUTHORIZED);
            };

            match super::jwt::decode_token(&secret, token) {
                Ok(claims) => {
                    let mut req = req;
                    req.extensions_mut().insert(AuthContext {
                        operator_id: claims.sub.parse().unwrap_or(0),
                        username: claims.username,
                        role: Role::from_i16(claims.role),
                    });
                    Ok(next.run(req).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        })
    }
}

/// 从请求 extensions 提取 AuthContext（handler 用）
pub fn context_from_request(req: &Request<axum::body::Body>) -> Option<&AuthContext> {
    req.extensions().get::<AuthContext>()
}
