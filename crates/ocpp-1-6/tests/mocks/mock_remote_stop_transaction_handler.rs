//! Mock RemoteStopTransaction Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::remote_stop_transaction::{RemoteStopTransactionHandler, RemoteStopTransactionRequest},
    messages::confs::remote_stop_transaction_conf::RemoteStopTransactionConfirmation,
    common::status::RemoteStartStopStatus,
};

#[derive(Debug, Clone)]
pub struct MockRemoteStopTransactionHandler {
    pub status: RemoteStartStopStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<RemoteStopTransactionRequest>>>,
}

impl MockRemoteStopTransactionHandler {
    pub fn accepted() -> Self {
        Self {
            status: RemoteStartStopStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: RemoteStartStopStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl RemoteStopTransactionHandler for MockRemoteStopTransactionHandler {
    fn handle(&self, req: RemoteStopTransactionRequest) -> RemoteStopTransactionConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        RemoteStopTransactionConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_remote_stop() {
        let handler = MockRemoteStopTransactionHandler::accepted();
        let req = RemoteStopTransactionRequest { transaction_id: 42 };
        let conf = handler.handle(req);
        assert_eq!(conf.status, RemoteStartStopStatus::Accepted);
    }
}
