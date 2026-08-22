//! HTTP Basic Auth 头解析
//!
//! OCPP 1.6 Profile 2（主流）：桩在 WebSocket upgrade 时发送
//! `Authorization: Basic base64(chargePointIdentity:password)`。

use base64::{Engine as _, engine::general_purpose::STANDARD};

/// 解析 `Authorization: Basic <base64>` 头
///
/// 返回 `(identity, password)`。格式错误 / 非 Basic 头 / 缺冒号 / 非 UTF-8 / base64 解码失败
/// 一律返回 `None`（调用方应判 401）。
pub fn parse_basic_auth(header_value: &str) -> Option<(String, String)> {
    let b64 = header_value.strip_prefix("Basic ")?.trim();
    if b64.is_empty() {
        return None;
    }
    let decoded = STANDARD.decode(b64).ok()?;
    let s = String::from_utf8(decoded).ok()?;
    let (identity, password) = s.split_once(':')?;
    if identity.is_empty() {
        return None;
    }
    Some((identity.to_string(), password.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_basic_auth() {
        // "CP001:secret123" base64 = "Q1AwMDE6c2VjcmV0MTIz"
        let (id, pw) = parse_basic_auth("Basic Q1AwMDE6c2VjcmV0MTIz").unwrap();
        assert_eq!(id, "CP001");
        assert_eq!(pw, "secret123");
    }

    #[test]
    fn handles_colon_in_password() {
        // "user:p:a:s:s" base64
        let s = base64::engine::general_purpose::STANDARD.encode(b"user:p:a:s:s");
        let (id, pw) = parse_basic_auth(&format!("Basic {s}")).unwrap();
        assert_eq!(id, "user");
        assert_eq!(pw, "p:a:s:s");
    }

    #[test]
    fn handles_empty_password() {
        // "CP001:" base64 = "Q1AwMDE6"
        let (id, pw) = parse_basic_auth("Basic Q1AwMDE6").unwrap();
        assert_eq!(id, "CP001");
        assert_eq!(pw, "");
    }

    #[test]
    fn rejects_bearer() {
        assert!(parse_basic_auth("Bearer xxx").is_none());
    }

    #[test]
    fn rejects_garbage_base64() {
        assert!(parse_basic_auth("Basic !!!notbase64!!!").is_none());
    }

    #[test]
    fn rejects_empty_prefix() {
        assert!(parse_basic_auth("Basic ").is_none());
        assert!(parse_basic_auth("Basic  ").is_none());
    }

    #[test]
    fn rejects_no_colon() {
        // "noColon" base64
        let s = base64::engine::general_purpose::STANDARD.encode(b"noColon");
        assert!(parse_basic_auth(&format!("Basic {s}")).is_none());
    }

    #[test]
    fn rejects_empty_identity() {
        // ":secret" base64
        let s = base64::engine::general_purpose::STANDARD.encode(b":secret");
        assert!(parse_basic_auth(&format!("Basic {s}")).is_none());
    }

    #[test]
    fn case_sensitive_prefix() {
        // RFC 7235 要求 "Basic" 大小写敏感（实现允许；这里我们严格）
        assert!(parse_basic_auth("basic Q1AwMDE6dGVzdA==").is_none());
    }
}