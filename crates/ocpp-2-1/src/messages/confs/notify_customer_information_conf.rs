//! NotifyCustomerInformation Confirmation (Block N)
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyCustomerInformationConfirmation {}

impl NotifyCustomerInformationConfirmation {
    pub fn new() -> Self { Self {} }
}
