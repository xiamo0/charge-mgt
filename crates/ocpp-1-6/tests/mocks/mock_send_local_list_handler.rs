//! Mock SendLocalList Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::send_local_list::{SendLocalListHandler, SendLocalListRequest},
    messages::confs::send_local_list_conf::SendLocalListConfirmation,
    common::status::UpdateStatus,
};

#[derive(Debug, Clone)]
pub struct MockSendLocalListHandler {
    pub status: UpdateStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<SendLocalListRequest>>>,
}

impl MockSendLocalListHandler {
    pub fn accepted() -> Self {
        Self {
            status: UpdateStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn failed() -> Self {
        Self {
            status: UpdateStatus::Failed,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn version_mismatch() -> Self {
        Self {
            status: UpdateStatus::VersionMismatch,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl SendLocalListHandler for MockSendLocalListHandler {
    fn handle(&self, req: SendLocalListRequest) -> SendLocalListConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        SendLocalListConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_1_6::common::status::UpdateType;
    use ocpp_1_6::common::authorization_list::AuthorizationVersion;

    #[test]
    fn test_mock_send_local_list() {
        let handler = MockSendLocalListHandler::accepted();
        let req = SendLocalListRequest {
            list_version: "1".to_string(),
            local_authorization_list: None,
            update_type: UpdateType::Differential,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, UpdateStatus::Accepted);
    }
}
