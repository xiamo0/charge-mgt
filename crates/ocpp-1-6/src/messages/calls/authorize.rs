//! Authorize 消息及处理器
//!
//! 定义授权请求（idTag）与处理器接口，默认实现始终返回 Accepted（模拟允许）。

use super::super::confs::authorize_conf::AuthorizeConfirmation;
use serde::{Deserialize, Serialize};

/// Authorize 请求，包含要进行授权的 idTag
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizeRequest {
    /// idTag 字符串，用于识别用户/卡片
    #[serde(rename = "idTag")]
    pub id_tag: String,
}

/// Authorize 的处理器接口，处理请求并返回确认
pub trait AuthorizeHandler: Send + Sync {
    fn handle(&self, req: AuthorizeRequest) -> AuthorizeConfirmation;
}

/// 默认的 Authorize 处理器（简单实现）
pub struct DefaultAuthorizeHandler;

impl DefaultAuthorizeHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl AuthorizeHandler for DefaultAuthorizeHandler {
    /// 处理授权请求，默认返回 accepted
    fn handle(&self, _req: AuthorizeRequest) -> AuthorizeConfirmation {
        AuthorizeConfirmation::accepted()
    }
}

impl Default for DefaultAuthorizeHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_authorize_request_roundtrip() {
        let req = AuthorizeRequest {
            id_tag: "TAG001".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let de: AuthorizeRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req.id_tag, de.id_tag);
    }

    #[test]
    fn test_default_authorize_handler_new() {
        let handler = DefaultAuthorizeHandler::new();
        let req = AuthorizeRequest {
            id_tag: "TAG001".to_string(),
        };
        let conf = handler.handle(req);
        assert_eq!(
            conf.status,
            crate::common::id_tag::AuthorizationStatus::Accepted
        );
    }

    #[test]
    fn test_default_authorize_handler_default() {
        let handler = DefaultAuthorizeHandler;
        let req = AuthorizeRequest {
            id_tag: "TAG001".to_string(),
        };
        let conf = handler.handle(req);
        assert_eq!(
            conf.status,
            crate::common::id_tag::AuthorizationStatus::Accepted
        );
    }

    #[test]
    fn test_default_authorize_handler_thread_safe() {
        let handler: Arc<dyn AuthorizeHandler> = Arc::new(DefaultAuthorizeHandler::new());
        let req = AuthorizeRequest {
            id_tag: "TAG001".to_string(),
        };
        let conf = handler.handle(req.clone());
        assert_eq!(
            conf.status,
            crate::common::id_tag::AuthorizationStatus::Accepted
        );
    }

    #[test]
    fn test_default_authorize_handler_returns_correct_type() {
        let handler = DefaultAuthorizeHandler::new();
        let req = AuthorizeRequest {
            id_tag: "TAG001".to_string(),
        };
        let conf = handler.handle(req);
        assert!(conf.id_tag_info.is_none());
    }
}
