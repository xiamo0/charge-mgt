//! LogStatusNotification Request (Functional Block D)
//! 日志上传状态通知

use serde::{Deserialize, Serialize};

/// 上传状态枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UploadLogStatusEnumType {
    BadMessage,
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    Uploaded,
    UploadFailure,
    Uploading,
    AcceptedCanceled,
}

/// LogStatusNotification 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    /// 上传状态
    pub status: UploadLogStatusEnumType,
    /// 请求 ID (可选, 关联 GetLog 的 requestId)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

impl LogStatusNotificationRequest {
    pub fn new(status: UploadLogStatusEnumType) -> Self {
        Self {
            status,
            request_id: None,
        }
    }

    pub fn with_request_id(mut self, request_id: i32) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

pub const ACTION: &str = "LogStatusNotification";