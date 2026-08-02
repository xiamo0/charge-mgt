// infra/http/mod.rs
use std::time::Duration;
use anyhow::Result;
use reqwest::Client;

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
        charge_point_id: &str,
        payload: &[u8],
    ) -> Result<Vec<u8>> {
        let url = format!("{}/ocpp/{}", gateway_http_url.trim_end_matches('/'), charge_point_id);
        let resp = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(payload.to_vec())
            .send()
            .await?;
        let status = resp.status();
        let body = resp.bytes().await?;
        if !status.is_success() {
            anyhow::bail!("OCPP HTTP 发送失败 {}: {}", status, String::from_utf8_lossy(&body));
        }
        Ok(body.to_vec())
    }
}