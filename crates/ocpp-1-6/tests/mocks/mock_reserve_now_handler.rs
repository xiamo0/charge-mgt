//! Mock ReserveNow Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::reserve_now::{ReserveNowHandler, ReserveNowRequest},
    messages::confs::reserve_now_conf::ReserveNowConfirmation,
    common::status::ReservationStatus,
};

#[derive(Debug, Clone)]
pub struct MockReserveNowHandler {
    pub status: ReservationStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<ReserveNowRequest>>>,
}

impl MockReserveNowHandler {
    pub fn accepted() -> Self {
        Self {
            status: ReservationStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: ReservationStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn occupied() -> Self {
        Self {
            status: ReservationStatus::Occupied,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ReserveNowHandler for MockReserveNowHandler {
    fn handle(&self, req: ReserveNowRequest) -> ReserveNowConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        ReserveNowConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_reserve_now() {
        let handler = MockReserveNowHandler::accepted();
        let req = ReserveNowRequest {
            connector_id: 1,
            expiry_date: "2024-01-01T12:00:00Z".to_string(),
            id_tag: "TAG001".to_string(),
            reservation_id: 1,
            parent_id_tag: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, ReservationStatus::Accepted);
    }
}
