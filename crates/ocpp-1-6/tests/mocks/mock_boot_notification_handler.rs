//! Mock BootNotification Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::boot_notification::{BootNotificationHandler, BootNotificationRequest},
    messages::confs::boot_notification_conf::BootNotificationConfirmation,
    common::status::RegistrationStatus,
};

#[derive(Debug, Clone)]
pub struct MockBootNotificationHandler {
    pub status: RegistrationStatus,
    pub interval_secs: i32,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<BootNotificationRequest>>>,
}

impl MockBootNotificationHandler {
    pub fn accepted(interval_secs: i32) -> Self {
        Self {
            status: RegistrationStatus::Accepted,
            interval_secs,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: RegistrationStatus::Rejected,
            interval_secs: 0,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn pending() -> Self {
        Self {
            status: RegistrationStatus::Pending,
            interval_secs: 60,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl BootNotificationHandler for MockBootNotificationHandler {
    fn handle(&self, req: BootNotificationRequest) -> BootNotificationConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        let now = chrono::Utc::now().to_rfc3339();
        BootNotificationConfirmation::accepted(&now, self.interval_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_boot_notification_accepted() {
        let handler = MockBootNotificationHandler::accepted(30);
        let req = BootNotificationRequest {
            charge_point_vendor: "VendorX".to_string(),
            charge_point_model: "ModelY".to_string(),
            charge_box_serial_number: None,
            charge_point_serial_number: None,
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, RegistrationStatus::Accepted);
        assert_eq!(conf.interval, 30);
    }
}
