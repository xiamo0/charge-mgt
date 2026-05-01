//! Mock RemoteStartTransaction Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::remote_start_transaction::{RemoteStartTransactionHandler, RemoteStartTransactionRequest},
    messages::confs::remote_start_transaction_conf::RemoteStartTransactionConfirmation,
    common::status::RemoteStartStopStatus,
};

#[derive(Debug, Clone)]
pub struct MockRemoteStartTransactionHandler {
    pub status: RemoteStartStopStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<RemoteStartTransactionRequest>>>,
}

impl MockRemoteStartTransactionHandler {
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

impl RemoteStartTransactionHandler for MockRemoteStartTransactionHandler {
    fn handle(&self, req: RemoteStartTransactionRequest) -> RemoteStartTransactionConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        RemoteStartTransactionConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_remote_start_accepted() {
        let handler = MockRemoteStartTransactionHandler::accepted();
        let req = RemoteStartTransactionRequest {
            connector_id: Some(1),
            id_tag: "TAG001".to_string(),
            charging_profile: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, RemoteStartStopStatus::Accepted);
    }
}
