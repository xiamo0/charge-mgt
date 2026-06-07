//! Heartbeat 消息及处理器
//! Heartbeat 消息及处理器
//!
//! Heartbeat 请求用于让后端获取充电点的当前时间（心跳），此处定义请求类型、处理器 trait 及默认实现。

use super::super::confs::heartbeat_conf::HeartbeatConfirmation;
use serde::{Deserialize, Serialize};

/// Heartbeat 请求类型（空结构体），序列化为 JSON 空对象
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct HeartbeatRequest;

impl HeartbeatRequest {
    /// 构造一个新的 HeartbeatRequest
    pub fn new() -> Self {
        Self
    }
}

/// Heartbeat 处理器 trait，处理请求并返回 HeartbeatConfirmation
pub trait HeartbeatHandler: Send + Sync {
    fn handle(&self, req: HeartbeatRequest) -> HeartbeatConfirmation;
}

/// 心跳处理相关配置
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    /// 心跳间隔（秒），用于控制本地触发心跳的频率
    pub interval_secs: u64,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self { interval_secs: 60 }
    }
}

/// 默认 Heartbeat 处理器，基于配置返回当前时间
pub struct DefaultHeartbeatHandler {
    config: HeartbeatConfig,
}

impl DefaultHeartbeatHandler {
    /// 创建一个默认处理器实例
    pub fn new(config: HeartbeatConfig) -> Self {
        Self { config }
    }
}

impl HeartbeatHandler for DefaultHeartbeatHandler {
    fn handle(&self, _req: HeartbeatRequest) -> HeartbeatConfirmation {
        HeartbeatConfirmation::new(&chrono::Utc::now().to_rfc3339())
    }
}

impl DefaultHeartbeatHandler {
    /// 返回配置中的心跳间隔（秒）
    pub fn interval(&self) -> u64 {
        self.config.interval_secs
    }
}
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_heartbeat_request_roundtrip() {
        let req = HeartbeatRequest::new();
        let json = serde_json::to_string(&req).unwrap();
        let de: HeartbeatRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, de);
    }

    #[test]
    fn test_heartbeat_config_default() {
        let config = HeartbeatConfig::default();
        assert_eq!(config.interval_secs, 60);
    }

    #[test]
    fn test_heartbeat_config_custom() {
        let config = HeartbeatConfig { interval_secs: 120 };
        assert_eq!(config.interval_secs, 120);
    }

    #[test]
    fn test_default_heartbeat_handler_new() {
        let config = HeartbeatConfig::default();
        let handler = DefaultHeartbeatHandler::new(config);
        assert_eq!(handler.interval(), 60);
    }

    #[test]
    fn test_default_heartbeat_handler_handle() {
        let config = HeartbeatConfig::default();
        let handler = DefaultHeartbeatHandler::new(config);
        let req = HeartbeatRequest::new();
        let conf = handler.handle(req);
        assert!(!conf.current_time.is_empty());
    }

    #[test]
    fn test_default_heartbeat_handler_thread_safe() {
        let config = HeartbeatConfig::default();
        let handler: Arc<dyn HeartbeatHandler> = Arc::new(DefaultHeartbeatHandler::new(config));
        let req = HeartbeatRequest::new();
        let conf = handler.handle(req);
        assert!(!conf.current_time.is_empty());
    }
}
