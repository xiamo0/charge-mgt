//! UpdateFirmware Confirmation (Block L)
use serde::{Deserialize, Serialize};
use crate::common::{UpdateFirmwareStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareConfirmation {
    pub status: UpdateFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
