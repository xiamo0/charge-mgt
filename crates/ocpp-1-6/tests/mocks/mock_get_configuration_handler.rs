//! Mock GetConfiguration Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::get_configuration::{GetConfigurationHandler, GetConfigurationRequest},
    messages::confs::get_configuration_conf::{GetConfigurationConfirmation, KeyValue},
};

#[derive(Debug, Clone)]
pub struct MockGetConfigurationHandler {
    pub call_count: Arc<AtomicUsize>,
    pub configuration: Vec<KeyValue>,
    pub unknown_keys: Vec<String>,
}

impl MockGetConfigurationHandler {
    pub fn new(configuration: Vec<KeyValue>, unknown_keys: Vec<String>) -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
            configuration,
            unknown_keys,
        }
    }

    pub fn empty() -> Self {
        Self::new(vec![], vec![])
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl GetConfigurationHandler for MockGetConfigurationHandler {
    fn handle(&self, _req: GetConfigurationRequest) -> GetConfigurationConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        GetConfigurationConfirmation {
            configuration: self.configuration.clone(),
            unknown_keys: self.unknown_keys.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_get_configuration() {
        let handler = MockGetConfigurationHandler::empty();
        let req = GetConfigurationRequest { key: None };
        let conf = handler.handle(req);
        assert!(conf.configuration.is_empty());
        assert!(conf.unknown_keys.is_empty());
    }
}
