// infra/http/mod.rs
use crate::error::AppError;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpSender {
    client: Client,
}

impl HttpSender {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .pool_idle_timeout(Duration::from_secs(60))
            .build()?;
        Ok(Self { client })
    }

    /// POST JSON 到指定桩的 HTTP 入口（gateway 暴露的 /ocpp/...）
    pub async fn post_ocpp(
        &self,
        gateway_http_url: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        let resp = self
            .client
            .post(gateway_http_url)
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(|e| AppError::OCPP_1_6_ERROR {
                action: "OCPP HTTP 发送失败".to_string(),
                detail: e.to_string(),
            })?;
        let status = resp.status();
        let body = resp.json().await.map_err(|e| AppError::OCPP_1_6_ERROR {
            action: "OCPP HTTP 发送失败".to_string(),
            detail: e.to_string(),
        })?;
        if !status.is_success() {
            return Err(AppError::OCPP_1_6_ERROR {
                action: "OCPP HTTP 发送失败".to_string(),
                detail: format!("OCPP HTTP 发送失败 {}: {:?}", status, body),
            });
        }
        Ok(body)
    }
}
