//! Heartbeat 响应
//!
//! Heartbeat 的确认消息，携带当前时间戳（RFC3339）。

use serde::{Deserialize, Serialize};

/// Heartbeat 的确认结构，包含服务器/设备当前时间字符串
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatConfirmation {
    /// 当前时间（RFC3339 字符串）
    pub current_time: String,
}

impl HeartbeatConfirmation {
    /// 使用给定的 RFC3339 字符串创建 HeartbeatConfirmation
    pub fn new(current_time: &str) -> Self {
        Self {
            current_time: current_time.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_confirmation_new() {
        let conf = HeartbeatConfirmation::new("2024-01-01T00:00:00Z");
        assert_eq!(conf.current_time, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_heartbeat_confirmation_roundtrip() {
        let conf = HeartbeatConfirmation::new("2024-12-31T23:59:59Z");
        let json = serde_json::to_string(&conf).unwrap();
        let de: HeartbeatConfirmation = serde_json::from_str(&json).unwrap();
        assert_eq!(conf.current_time, de.current_time);
    }
}
