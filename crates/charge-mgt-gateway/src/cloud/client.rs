use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::config::CloudConfig;
use crate::error::{GatewayError, Result};

#[derive(Debug, Clone)]
pub struct CloudApiClient {
    client: Client,
    api_url: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl CloudApiClient {
    pub fn new(config: &CloudConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| GatewayError::CloudApi(format!("Failed to create HTTP client: {}", e)))?;

        info!("Cloud API client created, url: {}", config.api_url);

        Ok(Self {
            client,
            api_url: config.api_url.clone(),
            api_key: config.api_key.clone(),
        })
    }

    pub async fn register_device(&self, device_id: &str, vendor: &str) -> Result<()> {
        let url = format!("{}/api/devices/register", self.api_url);

        #[derive(Serialize)]
        struct RegisterRequest {
            device_id: String,
            vendor: String,
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&RegisterRequest {
                device_id: device_id.to_string(),
                vendor: vendor.to_string(),
            })
            .send()
            .await
            .map_err(|e| GatewayError::CloudApi(format!("HTTP request failed: {}", e)))?;

        if response.status().is_success() {
            info!("Device registered: {}", device_id);
            Ok(())
        } else {
            let status = response.status();
            error!("Device registration failed: {} - {}", device_id, status);
            Err(GatewayError::CloudApi(format!(
                "Registration failed: {}",
                status
            )))
        }
    }

    pub async fn send_heartbeat(&self, device_id: &str) -> Result<()> {
        let url = format!("{}/api/devices/{}/heartbeat", self.api_url, device_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| GatewayError::CloudApi(format!("HTTP request failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(GatewayError::CloudApi(format!(
                "Heartbeat failed: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_api_client_new() {
        let config = CloudConfig {
            api_url: "https://cloud.example.com".to_string(),
            api_key: "test_key".to_string(),
        };

        let client = CloudApiClient::new(&config);
        assert!(client.is_ok());
    }
}
