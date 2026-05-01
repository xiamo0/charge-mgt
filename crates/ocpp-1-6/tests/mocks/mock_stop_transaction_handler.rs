//! Mock StopTransaction Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::stop_transaction::{StopTransactionHandler, StopTransactionRequest},
    messages::confs::stop_transaction_conf::StopTransactionConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockStopTransactionHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<StopTransactionRequest>>>,
}

impl MockStopTransactionHandler {
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

impl Default for MockStopTransactionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl StopTransactionHandler for MockStopTransactionHandler {
    fn handle(&self, req: StopTransactionRequest) -> StopTransactionConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        StopTransactionConfirmation::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_stop_transaction() {
        let handler = MockStopTransactionHandler::new();
        let req = StopTransactionRequest {
            meter_stop: 2000,
            timestamp: "2024-01-01T01:00:00Z".to_string(),
            transaction_id: 42,
            reason: None,
            id_tag: None,
            transaction_data: None,
        };
        let conf = handler.handle(req.clone());
        assert_eq!(handler.get_call_count(), 1);
        let last = handler.last_request.lock().unwrap();
        assert_eq!(last.as_ref().unwrap().transaction_id, 42);
    }
}
