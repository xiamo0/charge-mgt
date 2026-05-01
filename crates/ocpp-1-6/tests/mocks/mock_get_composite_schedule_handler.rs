//! Mock GetCompositeSchedule Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::get_composite_schedule::{GetCompositeScheduleHandler, GetCompositeScheduleRequest},
    messages::confs::get_composite_schedule_conf::GetCompositeScheduleConfirmation,
    common::status::GetCompositeScheduleStatus,
    common::configuration::ChargingSchedule,
};

#[derive(Debug, Clone)]
pub struct MockGetCompositeScheduleHandler {
    pub status: GetCompositeScheduleStatus,
    pub schedule: Option<ChargingSchedule>,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<GetCompositeScheduleRequest>>>,
}

impl MockGetCompositeScheduleHandler {
    pub fn accepted() -> Self {
        Self {
            status: GetCompositeScheduleStatus::Accepted,
            schedule: None,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: GetCompositeScheduleStatus::Rejected,
            schedule: None,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl GetCompositeScheduleHandler for MockGetCompositeScheduleHandler {
    fn handle(&self, req: GetCompositeScheduleRequest) -> GetCompositeScheduleConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        GetCompositeScheduleConfirmation {
            status: self.status.clone(),
            schedule: self.schedule.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_get_composite_schedule() {
        let handler = MockGetCompositeScheduleHandler::accepted();
        let req = GetCompositeScheduleRequest {
            connector_id: 1,
            duration_secs: 3600,
            charging_profile_purpose: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, GetCompositeScheduleStatus::Accepted);
    }
}
