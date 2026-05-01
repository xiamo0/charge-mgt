//! Mock UnlockConnector Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::unlock_connector::{UnlockConnectorHandler, UnlockConnectorRequest},
    messages::confs::unlock_connector_conf::UnlockConnectorConfirmation,
    common::status::UnlockStatus,
};

#[derive(Debug, Clone)]
pub struct MockUnlockConnectorHandler {
    pub status: UnlockStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<UnlockConnectorRequest>>>,
}

impl MockUnlockConnectorHandler {
    pub fn unlocked() -> Self {
        Self {
            status: UnlockStatus::Unlocked,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn unlock_failed() -> Self {
        Self {
            status: UnlockStatus::UnlockFailed,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn not_supported() -> Self {
        Self {
            status: UnlockStatus::NotSupported,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl UnlockConnectorHandler for MockUnlockConnectorHandler {
    fn handle(&self, req: UnlockConnectorRequest) -> UnlockConnectorConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        UnlockConnectorConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_unlock_connector() {
        let handler = MockUnlockConnectorHandler::unlocked();
        let req = UnlockConnectorRequest { connector_id: 1 };
        let conf = handler.handle(req);
        assert_eq!(conf.status, UnlockStatus::Unlocked);
    }
}
