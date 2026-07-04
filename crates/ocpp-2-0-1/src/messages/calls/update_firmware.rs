//! UpdateFirmware Request (Functional Block I)
//! 发起固件更新

use serde::{Deserialize, Serialize};

/// 固件类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    /// 下载 URL
    pub location: String,
    /// 检索日期
    pub retrieve_date_time: String,
    /// 安装日期 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<String>,
    /// 签名证书 (可选, max 5500)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    /// 签名 (可选, max 800)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// UpdateFirmware 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    /// 请求 ID
    pub request_id: i32,
    /// 固件信息
    pub firmware: FirmwareType,
    /// 重试次数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// 重试间隔 (秒, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

impl UpdateFirmwareRequest {
    pub fn new(request_id: i32, firmware: FirmwareType) -> Self {
        Self {
            request_id,
            firmware,
            retries: None,
            retry_interval: None,
        }
    }

    /// 设置重试参数
    pub fn with_retries(mut self, retries: i32, interval: i32) -> Self {
        self.retries = Some(retries);
        self.retry_interval = Some(interval);
        self
    }
}

pub const ACTION: &str = "UpdateFirmware";
