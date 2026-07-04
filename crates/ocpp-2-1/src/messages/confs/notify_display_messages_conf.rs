//! NotifyDisplayMessages Confirmation (Block O)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyDisplayMessagesConfirmation {}

impl NotifyDisplayMessagesConfirmation {
    pub fn new() -> Self { Self {} }
}
