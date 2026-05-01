//! Mock FirmwareStatusNotification Handler (VoidHandler - one-way)

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::firmware_status_notification::{FirmwareStatusNotificationHandler, FirmwareStatusNotificationRequest},
};

#[derive(Debug, Clone)]
pub struct MockFirmwareStatusNotificationHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<FirmwareStatusNotificationRequest>>>,
}

impl MockFirmwareStatusNotificationHandler {
    pub fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl Default for MockFirmwareStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl FirmwareStatusNotificationHandler for MockFirmwareStatusNotificationHandler {
    fn handle(&self, req: FirmwareStatusNotificationRequest) {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_1_6::common::status::FirmwareStatus;

    #[test]
    fn test_mock_firmware_status_notification() {
        let handler = MockFirmwareStatusNotificationHandler::new();
        let req = FirmwareStatusNotificationRequest {
            status: FirmwareStatus::Idle,
            request_id: None,
        };
        handler.handle(req);
        assert_eq!(handler.get_call_count(), 1);
    }
}
