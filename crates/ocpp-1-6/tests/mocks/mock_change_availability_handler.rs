//! Mock ChangeAvailability Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::change_availability::{ChangeAvailabilityHandler, ChangeAvailabilityRequest},
    messages::confs::change_availability_conf::ChangeAvailabilityConfirmation,
    common::status::{AvailabilityStatus, AvailabilityType},
};

#[derive(Debug, Clone)]
pub struct MockChangeAvailabilityHandler {
    pub status: AvailabilityStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<ChangeAvailabilityRequest>>>,
}

impl MockChangeAvailabilityHandler {
    pub fn accepted() -> Self {
        Self {
            status: AvailabilityStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: AvailabilityStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ChangeAvailabilityHandler for MockChangeAvailabilityHandler {
    fn handle(&self, req: ChangeAvailabilityRequest) -> ChangeAvailabilityConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        ChangeAvailabilityConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_change_availability() {
        let handler = MockChangeAvailabilityHandler::accepted();
        let req = ChangeAvailabilityRequest {
            connector_id: 1,
            availability_type: AvailabilityType::Operative,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, AvailabilityStatus::Accepted);
    }
}
