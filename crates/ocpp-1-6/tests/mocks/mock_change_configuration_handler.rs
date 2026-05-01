//! Mock ChangeConfiguration Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::change_configuration::{ChangeConfigurationHandler, ChangeConfigurationRequest},
    messages::confs::change_configuration_conf::ChangeConfigurationConfirmation,
    common::status::ConfigurationStatus,
};

#[derive(Debug, Clone)]
pub struct MockChangeConfigurationHandler {
    pub status: ConfigurationStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<ChangeConfigurationRequest>>>,
}

impl MockChangeConfigurationHandler {
    pub fn accepted() -> Self {
        Self {
            status: ConfigurationStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: ConfigurationStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn reboot_required() -> Self {
        Self {
            status: ConfigurationStatus::RebootRequired,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ChangeConfigurationHandler for MockChangeConfigurationHandler {
    fn handle(&self, req: ChangeConfigurationRequest) -> ChangeConfigurationConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        ChangeConfigurationConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_change_configuration() {
        let handler = MockChangeConfigurationHandler::accepted();
        let req = ChangeConfigurationRequest {
            key: "HeartbeatInterval".to_string(),
            value: "60".to_string(),
        };
        let conf = handler.handle(req);
        assert_eq!(conf.status, ConfigurationStatus::Accepted);
    }
}
