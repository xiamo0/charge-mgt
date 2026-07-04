//! NotifyDERAlarm Confirmation (Block R — 2.1)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyDERAlarmConfirmation {}

impl NotifyDERAlarmConfirmation {
    pub fn new() -> Self { Self {} }
}
