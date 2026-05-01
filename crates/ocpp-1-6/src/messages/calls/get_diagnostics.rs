//! GetDiagnostics 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::get_diagnostics_conf::GetDiagnosticsConfirmation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDiagnosticsRequest {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<String>,
}

pub trait GetDiagnosticsHandler: Send + Sync {
    fn handle(&self, req: GetDiagnosticsRequest) -> GetDiagnosticsConfirmation;
}

pub struct DefaultGetDiagnosticsHandler;

impl Default for DefaultGetDiagnosticsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetDiagnosticsHandler {
    pub fn new() -> Self {
        Self
    }
}

impl GetDiagnosticsHandler for DefaultGetDiagnosticsHandler {
    fn handle(&self, _req: GetDiagnosticsRequest) -> GetDiagnosticsConfirmation {
        GetDiagnosticsConfirmation::empty()
    }
}