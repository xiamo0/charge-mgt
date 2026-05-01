//! Mock ClearChargingProfile Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::clear_charging_profile::{ClearChargingProfileHandler, ClearChargingProfileRequest},
    messages::confs::clear_charging_profile_conf::ClearChargingProfileConfirmation,
    common::status::ClearChargingProfileStatus,
};

#[derive(Debug, Clone)]
pub struct MockClearChargingProfileHandler {
    pub status: ClearChargingProfileStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<ClearChargingProfileRequest>>>,
}

impl MockClearChargingProfileHandler {
    pub fn accepted() -> Self {
        Self {
            status: ClearChargingProfileStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn unknown() -> Self {
        Self {
            status: ClearChargingProfileStatus::Unknown,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ClearChargingProfileHandler for MockClearChargingProfileHandler {
    fn handle(&self, req: ClearChargingProfileRequest) -> ClearChargingProfileConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        ClearChargingProfileConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_clear_charging_profile() {
        let handler = MockClearChargingProfileHandler::accepted();
        let req = ClearChargingProfileRequest {
            connector_id: None,
            charging_profile_purpose: None,
            stack_level: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, ClearChargingProfileStatus::Accepted);
    }
}
