//! NotifyAllowedEnergyTransfer Confirmation (Block Q — 2.1)
use serde::{Deserialize, Serialize};
use crate::common::{NotifyAllowedEnergyTransferStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferConfirmation {
    pub status: NotifyAllowedEnergyTransferStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
