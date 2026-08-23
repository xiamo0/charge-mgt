//! Basic Auth 凭据验证（通过 cloud 的 /internal/auth/verify 端点）
//!
//! 设计：gateway 不直接查 DB，而是调用 cloud 的内网 API 验证桩密码。
//! - 200 + `{"valid": true}`  → 密码正确
//! - 200 + `{"valid": false}` → 密码错（或桩不存在，不区分）
//! - 401                    → 内网鉴权失败（api_key 不匹配）
//! - 网络错/5xx             → 调用失败（调用方应 fail-closed）

use serde::Deserialize;

use crate::config::CloudConfig;
use crate::error::GatewayError;

#[derive(Debug, Deserialize)]
struct VerifyResponse {
    valid: bool,
}

/// 通过 cloud 的 internal 端点验证桩的 Basic Auth 密码
///
/// 返回：
/// - `Ok(true)`  密码正确
/// - `Ok(false)` 密码错 / 桩不存在（不区分）
/// - `Err(e)`    网络错 / cloud 返回 5xx（调用方 fail-closed）
pub async fn verify_via_cloud(
    cloud: &CloudConfig,
    http: &reqwest::Client,
    charge_point_id: &str,
    password: &str,
) -> Result<bool, GatewayError> {
    let url = format!(
        "{}/internal/auth/verify",
        cloud.api_url.trim_end_matches('/')
    );

    let resp = http
        .post(&url)
        .header("Authorization", format!("Bearer {}", cloud.api_key))
        .json(&serde_json::json!({
            "charge_point_id": charge_point_id,
            "password": password,
        }))
        .send()
        .await
        .map_err(|e| GatewayError::Auth(format!("调用 cloud verify-auth 失败: {e}")))?;

    match resp.status().as_u16() {
        200 => {
            let body: VerifyResponse = resp
                .json()
                .await
                .map_err(|e| GatewayError::Auth(format!("解析 verify-auth 响应: {e}")))?;
            Ok(body.valid)
        }
        401 => Ok(false), // 内网鉴权失败 = 凭据错
        other => Err(GatewayError::Auth(format!(
            "cloud verify-auth 返回异常状态码 {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cloud() -> CloudConfig {
        CloudConfig {
            api_url: "http://127.0.0.1:0".into(), // 无效端口，测试网络错路径
            api_key: "test-key".into(),
        }
    }

    #[tokio::test]
    async fn network_error_returns_err() {
        let cloud = test_cloud();
        let http = reqwest::Client::new();
        let result = verify_via_cloud(&cloud, &http, "CP001", "pw").await;
        assert!(result.is_err(), "网络错应该返回 Err（fail-closed）");
    }
}
