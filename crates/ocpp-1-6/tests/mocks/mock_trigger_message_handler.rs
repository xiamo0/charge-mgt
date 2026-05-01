//! Mock TriggerMessage Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::trigger_message::{TriggerMessageHandler, TriggerMessageRequest},
    messages::confs::trigger_message_conf::TriggerMessageConfirmation,
    common::status::TriggerMessageStatus,
};

#[derive(Debug, Clone)]
pub struct MockTriggerMessageHandler {
    pub status: TriggerMessageStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<TriggerMessageRequest>>>,
}

impl MockTriggerMessageHandler {
    pub fn accepted() -> Self {
        Self {
            status: TriggerMessageStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: TriggerMessageStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn not_implemented() -> Self {
        Self {
            status: TriggerMessageStatus::NotImplemented,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl TriggerMessageHandler for MockTriggerMessageHandler {
    fn handle(&self, req: TriggerMessageRequest) -> TriggerMessageConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        TriggerMessageConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ocpp_1_6::common::status::MessageTrigger;

    #[test]
    fn test_mock_trigger_message() {
        let handler = MockTriggerMessageHandler::accepted();
        let req = TriggerMessageRequest {
            requested_message: MessageTrigger::Heartbeat,
            connector_id: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, TriggerMessageStatus::Accepted);
    }
}
