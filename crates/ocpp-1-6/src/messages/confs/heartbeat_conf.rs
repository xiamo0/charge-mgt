//! Heartbeat 响应

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatConfirmation {
    pub current_time: String,
}

impl HeartbeatConfirmation {
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
