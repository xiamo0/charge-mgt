//! 云端 REST API 客户端
//!
//! 负责与云端服务进行 HTTP 通信，包括设备注册与心跳上报。

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::config::CloudConfig;
use crate::error::{GatewayError, Result};

/// 云端 API 客户端，使用 Bearer Token 认证
#[derive(Debug, Clone)]
pub struct CloudApiClient {
    /// HTTP 客户端实例
    client: Client,
    /// 云端 API 基础 URL
    api_url: String,
    /// API 认证密钥
    api_key: String,
}

/// 云端 API 通用响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 请求是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<T>,
    /// 错误信息（失败时）
    pub error: Option<String>,
}

impl CloudApiClient {
    /// 根据配置创建 HTTP 客户端，请求超时 30 秒
    pub fn new(config: &CloudConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| GatewayError::CloudApi(format!("创建 HTTP 客户端失败: {}", e)))?;

        info!("云端 API 客户端已创建，地址: {}", config.api_url);

        Ok(Self {
            client,
            api_url: config.api_url.clone(),
            api_key: config.api_key.clone(),
        })
    }

    /// 向云端注册充电桩设备
    pub async fn register_device(&self, device_id: &str, vendor: &str) -> Result<()> {
        let url = format!("{}/api/devices/register", self.api_url);

        #[derive(Serialize)]
        struct RegisterRequest {
            /// 设备 ID
            device_id: String,
            /// 设备厂商
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
            .map_err(|e| GatewayError::CloudApi(format!("HTTP 请求失败: {}", e)))?;

        if response.status().is_success() {
            info!("设备注册成功: {}", device_id);
            Ok(())
        } else {
            let status = response.status();
            error!("设备注册失败: {} - {}", device_id, status);
            Err(GatewayError::CloudApi(format!(
                "设备注册失败: {}",
                status
            )))
        }
    }

    /// 向云端发送设备心跳，维持在线状态
    pub async fn send_heartbeat(&self, device_id: &str) -> Result<()> {
        let url = format!("{}/api/devices/{}/heartbeat", self.api_url, device_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| GatewayError::CloudApi(format!("HTTP 请求失败: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(GatewayError::CloudApi(format!(
                "心跳发送失败: {}",
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
