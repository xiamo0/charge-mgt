//! OCPP 配置

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 全局 OCPP 配置，供所有处理器共享
#[derive(Debug, Clone)]
pub struct OcppConfig {
    pub heartbeat_interval_secs: u64,
    pub boot_status: crate::common::status::RegistrationStatus,
    pub boot_interval_secs: u64,
    pub timezone: String,
}

impl Default for OcppConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval_secs: 60,
            boot_status: crate::common::status::RegistrationStatus::Accepted,
            boot_interval_secs: 30,
            timezone: "UTC".to_string(),
        }
    }
}

/// 线程安全的配置包装器
pub type SharedConfig = Arc<OcppConfig>;

impl OcppConfig {
    pub fn shared(self) -> SharedConfig {
        Arc::new(self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Default)]
pub struct HandlerConfig {
    pub vendor_id: String,
    pub message_id: Option<String>,
}