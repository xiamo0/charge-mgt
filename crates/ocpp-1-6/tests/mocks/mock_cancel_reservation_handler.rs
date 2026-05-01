//! Mock CancelReservation Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::cancel_reservation::{CancelReservationHandler, CancelReservationRequest},
    messages::confs::cancel_reservation_conf::CancelReservationConfirmation,
    common::status::CancelReservationStatus,
};

#[derive(Debug, Clone)]
pub struct MockCancelReservationHandler {
    pub status: CancelReservationStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<CancelReservationRequest>>>,
}

impl MockCancelReservationHandler {
    pub fn accepted() -> Self {
        Self {
            status: CancelReservationStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: CancelReservationStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl CancelReservationHandler for MockCancelReservationHandler {
    fn handle(&self, req: CancelReservationRequest) -> CancelReservationConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        CancelReservationConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_cancel_reservation() {
        let handler = MockCancelReservationHandler::accepted();
        let req = CancelReservationRequest { reservation_id: 1 };
        let conf = handler.handle(req);
        assert_eq!(conf.status, CancelReservationStatus::Accepted);
    }
}
