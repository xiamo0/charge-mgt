//! NotifyAllowedEnergyTransfer Confirmation (Block Q — 2.1)
use crate::common::{NotifyAllowedEnergyTransferStatusEnumType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferConfirmation {
    pub status: NotifyAllowedEnergyTransferStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
