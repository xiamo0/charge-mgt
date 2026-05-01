//! Mock Heartbeat Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::heartbeat::{HeartbeatHandler, HeartbeatRequest},
    messages::confs::heartbeat_conf::HeartbeatConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockHeartbeatHandler {
    pub interval_secs: u64,
    pub call_count: Arc<AtomicUsize>,
}

impl MockHeartbeatHandler {
    pub fn new(interval_secs: u64) -> Self {
        Self {
            interval_secs,
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl HeartbeatHandler for MockHeartbeatHandler {
    fn handle(&self, _req: HeartbeatRequest) -> HeartbeatConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        HeartbeatConfirmation::new(&chrono::Utc::now().to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_heartbeat() {
        let handler = MockHeartbeatHandler::new(60);
        let req = HeartbeatRequest::new();
        let conf = handler.handle(req);
        assert!(!conf.current_time.is_empty());
        assert_eq!(handler.get_call_count(), 1);
    }
}
