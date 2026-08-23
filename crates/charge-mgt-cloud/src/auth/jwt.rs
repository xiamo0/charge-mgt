//! JWT 生成与校验（HS256）

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::ocpp16::entity::operator::Role;

/// JWT claims：sub=operator id, username, role, exp/iat 由库处理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// operator id（字符串形式）
    pub sub: String,
    pub username: String,
    /// 0=admin, 1=operator, 2=viewer
    pub role: i16,
    pub exp: usize,
    pub iat: usize,
}

/// 生成 JWT（HS256）
pub fn encode_token(
    secret: &str,
    operator_id: i64,
    username: &str,
    role: Role,
    ttl_secs: u64,
) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: operator_id.to_string(),
        username: username.to_string(),
        role: role as i16,
        iat: now,
        exp: now + ttl_secs as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT 生成失败: {e}")))
}

/// 校验 JWT，返回 claims
pub fn decode_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|e| AppError::Unauthorized(format!("JWT 校验失败: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test-secret-0123456789abcdef";

    #[test]
    fn roundtrip() {
        let token = encode_token(SECRET, 1, "admin", Role::Admin, 3600).unwrap();
        let claims = decode_token(SECRET, &token).unwrap();
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username, "admin");
        assert_eq!(claims.role, 0);
    }

    #[test]
    fn wrong_secret_fails() {
        let token = encode_token(SECRET, 1, "admin", Role::Admin, 3600).unwrap();
        assert!(decode_token("other-secret", &token).is_err());
    }

    #[test]
    fn expired_fails() {
        // exp 设为过去 1 小时，超过默认 60s leeway
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: "1".into(),
            username: "admin".into(),
            role: 0,
            iat: now - 7200,
            exp: now - 3600,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert!(decode_token(SECRET, &token).is_err());
    }
}
