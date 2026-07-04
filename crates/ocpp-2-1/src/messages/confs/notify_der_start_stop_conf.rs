//! NotifyDERStartStop Confirmation (Block R — 2.1)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyDERStartStopConfirmation {}

impl NotifyDERStartStopConfirmation {
    pub fn new() -> Self { Self {} }
}
