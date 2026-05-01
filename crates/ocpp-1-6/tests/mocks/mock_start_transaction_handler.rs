//! Mock StartTransaction Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::start_transaction::{StartTransactionHandler, StartTransactionRequest},
    messages::confs::start_transaction_conf::StartTransactionConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockStartTransactionHandler {
    pub transaction_id: i32,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<StartTransactionRequest>>>,
}

impl MockStartTransactionHandler {
    pub fn new(transaction_id: i32) -> Self {
        Self {
            transaction_id,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl StartTransactionHandler for MockStartTransactionHandler {
    fn handle(&self, req: StartTransactionRequest) -> StartTransactionConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        StartTransactionConfirmation::new(self.transaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_start_transaction() {
        let handler = MockStartTransactionHandler::new(42);
        let req = StartTransactionRequest {
            connector_id: 1,
            id_tag: "TAG001".to_string(),
            meter_start: 1000,
            reservation_id: None,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };
        let conf = handler.handle(req);
        assert_eq!(conf.transaction_id, 42);
        assert_eq!(handler.get_call_count(), 1);
    }
}
