//! OCPP 2.0.1 / 2.1 Diagnostics & Event types (Functional Block N)

use crate::common::{ComponentType, VariableType};
use serde::{Deserialize, Serialize};

/// 日志类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LogEnumType {
    DiagnosticsLog,
    SecurityLog,
}

/// 日志参数类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    pub remote_location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
}

/// 日志上传状态枚举
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

/// 事件触发枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventTriggerEnumType {
    Alerting,
    Delta,
    Periodic,
}

/// 事件通知类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventNotificationEnumType {
    HardWiredNotification,
    HardWiredMonitor,
    PreconfiguredMonitor,
    CustomMonitor,
}

/// 事件数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    pub event_id: i32,
    pub timestamp: String,
    pub trigger: EventTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub component: ComponentType,
    pub variable: VariableType,
    pub event_notification_type: EventNotificationEnumType,
}
