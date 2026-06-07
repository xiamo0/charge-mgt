//! OCPP 配置

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 全局 OCPP 配置，供所有处理器共享
#[derive(Debug, Clone)]
pub struct OcppConfig {
    /// 心跳间隔（秒）
    pub heartbeat_interval_secs: u64,
    /// 启动时返回给集线器的注册状态（Accepted/Pending/Rejected）
    pub boot_status: crate::common::status::RegistrationStatus,
    /// 启动确认中的间隔（秒）
    pub boot_interval_secs: u64,
    /// 时区标识（例如 UTC）
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

/// 线程安全的配置包装器（Arc）
pub type SharedConfig = Arc<OcppConfig>;

impl OcppConfig {
    pub fn shared(self) -> SharedConfig {
        Arc::new(self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct HandlerConfig {
    /// 处理器厂商 ID（可用于定位设备厂商特性）
    pub vendor_id: String,
    /// 可选的 message id，用于调试或链路追踪
    pub message_id: Option<String>,
}
