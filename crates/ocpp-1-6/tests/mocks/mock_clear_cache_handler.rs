//! Mock ClearCache Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::clear_cache::{ClearCacheHandler, ClearCacheRequest},
    messages::confs::clear_cache_conf::ClearCacheConfirmation,
    common::status::ClearCacheStatus,
};

#[derive(Debug, Clone)]
pub struct MockClearCacheHandler {
    pub status: ClearCacheStatus,
    pub call_count: Arc<AtomicUsize>,
}

impl MockClearCacheHandler {
    pub fn accepted() -> Self {
        Self {
            status: ClearCacheStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            status: ClearCacheStatus::Rejected,
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ClearCacheHandler for MockClearCacheHandler {
    fn handle(&self, _req: ClearCacheRequest) -> ClearCacheConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        ClearCacheConfirmation {
            status: self.status.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_clear_cache() {
        let handler = MockClearCacheHandler::accepted();
        let req = ClearCacheRequest::new();
        let conf = handler.handle(req);
        assert_eq!(conf.status, ClearCacheStatus::Accepted);
    }
}
