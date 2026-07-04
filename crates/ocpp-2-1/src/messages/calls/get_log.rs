//! GetLog Request (Block N)
use crate::common::{LogEnumType, LogParametersType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    pub log_type: LogEnumType,
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    pub log: LogParametersType,
}

pub const ACTION: &str = "GetLog";
