//! Mock GetDiagnostics Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::get_diagnostics::{GetDiagnosticsHandler, GetDiagnosticsRequest},
    messages::confs::get_diagnostics_conf::GetDiagnosticsConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockGetDiagnosticsHandler {
    pub filename: Option<String>,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<GetDiagnosticsRequest>>>,
}

impl MockGetDiagnosticsHandler {
    pub fn with_filename(filename: &str) -> Self {
        Self {
            filename: Some(filename.to_string()),
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn empty() -> Self {
        Self {
            filename: None,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl GetDiagnosticsHandler for MockGetDiagnosticsHandler {
    fn handle(&self, req: GetDiagnosticsRequest) -> GetDiagnosticsConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        GetDiagnosticsConfirmation::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_get_diagnostics() {
        let handler = MockGetDiagnosticsHandler::with_filename("/tmp/diagnostics.zip");
        let req = GetDiagnosticsRequest {
            location: "https://example.com/diagnostics".to_string(),
            retries: None,
            retry_interval: None,
            start_time: None,
            stop_time: None,
        };
        let conf = handler.handle(req);
        assert!(conf.filename.is_none());
    }
}
