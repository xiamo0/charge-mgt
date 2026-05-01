//! Mock SetChargingProfile Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::set_charging_profile::{SetChargingProfileHandler, SetChargingProfileRequest},
    messages::confs::set_charging_profile_conf::SetChargingProfileConfirmation,
    common::status::ChargingProfileStatus,
};

#[derive(Debug, Clone)]
pub struct MockSetChargingProfileHandler {
    pub status: ChargingProfileStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<SetChargingProfileRequest>>>,
}

impl MockSetChargingProfileHandler {
    pub fn accepted() -> Self {
        Self {
            status: ChargingProfileStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: ChargingProfileStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn not_supported() -> Self {
        Self {
            status: ChargingProfileStatus::NotSupported,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl SetChargingProfileHandler for MockSetChargingProfileHandler {
    fn handle(&self, req: SetChargingProfileRequest) -> SetChargingProfileConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        SetChargingProfileConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_set_charging_profile() {
        let handler = MockSetChargingProfileHandler::accepted();
        let req = SetChargingProfileRequest {
            connector_id: 1,
            charging_profile: ocpp_1_6::common::configuration::ChargingProfile {
                charging_profile_id: 1,
                stack_level: 0,
                charging_profile_purpose: "TxDefaultProfile".to_string(),
                charging_profile_kind: "Absolute".to_string(),
                valid_from: None,
                valid_to: None,
                charging_schedule: None,
                recurrency_kind: None,
                transaction_id: None,
            },
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, ChargingProfileStatus::Accepted);
    }
}
