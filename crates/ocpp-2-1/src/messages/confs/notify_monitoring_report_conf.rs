//! NotifyMonitoringReport Confirmation (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyMonitoringReportConfirmation {}

impl NotifyMonitoringReportConfirmation {
    pub fn new() -> Self { Self {} }
}
