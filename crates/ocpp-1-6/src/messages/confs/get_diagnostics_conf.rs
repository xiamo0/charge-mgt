//! GetDiagnostics 响应

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDiagnosticsConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl GetDiagnosticsConfirmation {
    pub fn empty() -> Self {
        Self { filename: None }
    }
}
