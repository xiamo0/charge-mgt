//! 内网 API 鉴权中间件
//!
//! 校验 `Authorization: Bearer <api_key>` header，
//! api_key 必须与 `AppState.config.cloud.api_key` 完全一致（constant-time 比较）。

use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// 验证内网请求的 Bearer token，失败返 401
///
/// 设计：api_key 通过闭包捕获传入，避免依赖 axum 0.4 缺失的 from_fn_with_state。
pub fn require_internal_token<B>(
    api_key: String,
) -> impl Fn(Request<B>, Next<B>) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
> + Clone
where
    B: Send + 'static,
{
    move |req: Request<B>, next: Next<B>| {
        let api_key = api_key.clone();
        Box::pin(async move {
            let provided = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .map(|s| s.as_bytes().to_vec());

            let expected = api_key.as_bytes().to_vec();
            let valid = match provided {
                Some(p) => expected.len() == p.len() && const_eq(&expected, &p),
                None => false,
            };

            if valid {
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        })
    }
}

/// constant-time byte 比较（防时间侧信道）
fn const_eq(a: &[u8], b: &[u8]) -> bool {
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff |= (a.len() ^ b.len()) as u8;
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::const_eq;

    #[test]
    fn const_eq_matches() {
        assert!(const_eq(b"hello", b"hello"));
    }

    #[test]
    fn const_eq_mismatch() {
        assert!(!const_eq(b"hello", b"world"));
    }

    #[test]
    fn const_eq_length_diff() {
        assert!(!const_eq(b"hello", b"helloo"));
        assert!(!const_eq(b"helloo", b"hello"));
    }
}
