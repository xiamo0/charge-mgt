//! Mock StatusNotification Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::status_notification::{StatusNotificationHandler, StatusNotificationRequest},
    messages::confs::status_notification_conf::StatusNotificationConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockStatusNotificationHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<StatusNotificationRequest>>>,
}

impl MockStatusNotificationHandler {
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

impl Default for MockStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusNotificationHandler for MockStatusNotificationHandler {
    fn handle(&self, req: StatusNotificationRequest) -> StatusNotificationConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        StatusNotificationConfirmation::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_1_6::common::status::{ChargePointStatus, ChargePointErrorCode};

    #[test]
    fn test_mock_status_notification() {
        let handler = MockStatusNotificationHandler::new();
        let req = StatusNotificationRequest {
            connector_id: 1,
            error_code: ChargePointErrorCode::NoError,
            info: None,
            status: ChargePointStatus::Available,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            vendor_id: None,
            vendor_error_code: None,
        };
        let _conf = handler.handle(req);
        assert_eq!(handler.get_call_count(), 1);
    }
}
