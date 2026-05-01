//! Mock DiagnosticsStatusNotification Handler (VoidHandler - one-way)

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::diagnostics_status_notification::{DiagnosticsStatusNotificationHandler, DiagnosticsStatusNotificationRequest},
};

#[derive(Debug, Clone)]
pub struct MockDiagnosticsStatusNotificationHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<DiagnosticsStatusNotificationRequest>>>,
}

impl MockDiagnosticsStatusNotificationHandler {
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

impl Default for MockDiagnosticsStatusNotificationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsStatusNotificationHandler for MockDiagnosticsStatusNotificationHandler {
    fn handle(&self, req: DiagnosticsStatusNotificationRequest) {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_1_6::common::status::DiagnosticsStatus;

    #[test]
    fn test_mock_diagnostics_status_notification() {
        let handler = MockDiagnosticsStatusNotificationHandler::new();
        let req = DiagnosticsStatusNotificationRequest {
            status: DiagnosticsStatus::Idle,
            request_id: None,
        };
        handler.handle(req);
        assert_eq!(handler.get_call_count(), 1);
    }
}
