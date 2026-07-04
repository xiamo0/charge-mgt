//! NotifyReport Confirmation (Block B)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyReportConfirmation {}

impl NotifyReportConfirmation {
    pub fn new() -> Self { Self {} }
}
