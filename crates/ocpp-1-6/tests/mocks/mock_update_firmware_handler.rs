//! Mock UpdateFirmware Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::update_firmware::{UpdateFirmwareHandler, UpdateFirmwareRequest},
    messages::confs::update_firmware_conf::UpdateFirmwareConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockUpdateFirmwareHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<UpdateFirmwareRequest>>>,
}

impl MockUpdateFirmwareHandler {
    pub fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl Default for MockUpdateFirmwareHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateFirmwareHandler for MockUpdateFirmwareHandler {
    fn handle(&self, req: UpdateFirmwareRequest) -> UpdateFirmwareConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        UpdateFirmwareConfirmation::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_update_firmware() {
        let handler = MockUpdateFirmwareHandler::new();
        let req = UpdateFirmwareRequest {
            location: "https://example.com/firmware.bin".to_string(),
            retries: Some(3),
            retry_interval: Some(60),
            request_id: 1,
            retrieve_time: "2024-01-01T00:00:00Z".to_string(),
        };
        let _conf = handler.handle(req);
        assert_eq!(handler.get_call_count(), 1);
    }
}
