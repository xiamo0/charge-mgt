//! GetLog Request (Functional Block D)
//! 请求上传日志

use serde::{Deserialize, Serialize};

/// 日志类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}

/// 日志参数类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    /// 远程位置 URL
    pub remote_location: String,
    /// 最旧日期 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<String>,
    /// 最新日期 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<String>,
}

/// GetLog 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    /// 日志类型
    pub log_type: LogEnumType,
    /// 请求 ID
    pub request_id: i32,
    /// 日志参数
    pub log: LogParametersType,
    /// 重试次数 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// 重试间隔 (秒, 可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

impl GetLogRequest {
    pub fn new(log_type: LogEnumType, request_id: i32, log: LogParametersType) -> Self {
        Self {
            log_type,
            request_id,
            log,
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

pub const ACTION: &str = "GetLog";
