//! Mock GetLocalListVersion Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::get_local_list_version::{GetLocalListVersionHandler, GetLocalListVersionRequest},
    messages::confs::get_local_list_version_conf::GetLocalListVersionConfirmation,
};

#[derive(Debug, Clone)]
pub struct MockGetLocalListVersionHandler {
    pub list_version: i32,
    pub call_count: Arc<AtomicUsize>,
}

impl MockGetLocalListVersionHandler {
    pub fn new(list_version: i32) -> Self {
        Self {
            list_version,
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl GetLocalListVersionHandler for MockGetLocalListVersionHandler {
    fn handle(&self, _req: GetLocalListVersionRequest) -> GetLocalListVersionConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        GetLocalListVersionConfirmation {
            list_version: self.list_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_get_local_list_version() {
        let handler = MockGetLocalListVersionHandler::new(5);
        let req = GetLocalListVersionRequest::new();
        let conf = handler.handle(req);
        assert_eq!(conf.list_version, 5);
    }
}
