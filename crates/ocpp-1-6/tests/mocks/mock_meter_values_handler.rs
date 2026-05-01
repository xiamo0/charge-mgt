//! Mock MeterValues Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::meter_values::{MeterValuesHandler, MeterValuesRequest},
    messages::confs::meter_values_conf::MeterValuesConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockMeterValuesHandler {
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<MeterValuesRequest>>>,
}

impl MockMeterValuesHandler {
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

impl Default for MockMeterValuesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MeterValuesHandler for MockMeterValuesHandler {
    fn handle(&self, req: MeterValuesRequest) -> MeterValuesConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req);
        MeterValuesConfirmation::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_meter_values() {
        let handler = MockMeterValuesHandler::new();
        let req = MeterValuesRequest {
            connector_id: 1,
            transaction_id: Some(42),
            meter_value: vec![],
        };
        let _conf = handler.handle(req);
        assert_eq!(handler.get_call_count(), 1);
    }
}
