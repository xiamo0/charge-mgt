//! Heartbeat 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::heartbeat_conf::HeartbeatConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct HeartbeatRequest;

impl HeartbeatRequest {
    pub fn new() -> Self {
        Self
    }
}

pub trait HeartbeatHandler: Send + Sync {
    fn handle(&self, req: HeartbeatRequest) -> HeartbeatConfirmation;
}

#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    pub interval_secs: u64,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self { interval_secs: 60 }
    }
}

pub struct DefaultHeartbeatHandler {
    config: HeartbeatConfig,
}

impl DefaultHeartbeatHandler {
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
    pub fn interval(&self) -> u64 {
        self.config.interval_secs
    }
}

#[cfg(test)]
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