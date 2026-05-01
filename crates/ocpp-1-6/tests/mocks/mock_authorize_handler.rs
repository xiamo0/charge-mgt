//! Mock Authorize Handler

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use ocpp_1_6::{
    messages::calls::authorize::{AuthorizeHandler, AuthorizeRequest},
    messages::confs::authorize_conf::AuthorizeConfirmation,
    common::id_tag::{AuthorizationStatus, IdTagInfo},
};

#[derive(Debug, Clone)]
pub struct MockAuthorizeHandler {
    pub auth_status: AuthorizationStatus,
    pub call_count: Arc<AtomicUsize>,
    pub last_request: Arc<std::sync::Mutex<Option<AuthorizeRequest>>>,
}

impl MockAuthorizeHandler {
    pub fn accepted() -> Self {
        Self {
            auth_status: AuthorizationStatus::Accepted,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn rejected() -> Self {
        Self {
            auth_status: AuthorizationStatus::Blocked,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn expired() -> Self {
        Self {
            auth_status: AuthorizationStatus::Expired,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn invalid() -> Self {
        Self {
            auth_status: AuthorizationStatus::Invalid,
            call_count: Arc::new(AtomicUsize::new(0)),
            last_request: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    pub fn get_last_request(&self) -> Option<AuthorizeRequest> {
        self.last_request.lock().unwrap().clone()
    }
}

impl AuthorizeHandler for MockAuthorizeHandler {
    fn handle(&self, req: AuthorizeRequest) -> AuthorizeConfirmation {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        *self.last_request.lock().unwrap() = Some(req.clone());
        AuthorizeConfirmation {
            status: self.auth_status.clone(),
            id_tag_info: Some(IdTagInfo::accepted()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_authorize_handler_accepted() {
        let handler = MockAuthorizeHandler::accepted();
        let req = AuthorizeRequest { id_tag: "TAG001".to_string() };
        let conf = handler.handle(req);
        assert_eq!(conf.status, AuthorizationStatus::Accepted);
        assert_eq!(handler.get_call_count(), 1);
    }

    #[test]
    fn test_mock_authorize_handler_rejected() {
        let handler = MockAuthorizeHandler::rejected();
        let req = AuthorizeRequest { id_tag: "TAG002".to_string() };
        let conf = handler.handle(req);
        assert_eq!(conf.status, AuthorizationStatus::Blocked);
    }

    #[test]
    fn test_mock_authorize_handler_captures_request() {
        let handler = MockAuthorizeHandler::accepted();
        let req = AuthorizeRequest { id_tag: "TAG003".to_string() };
        handler.handle(req.clone());
        let last = handler.get_last_request().unwrap();
        assert_eq!(last.id_tag, "TAG003");
    }
}
