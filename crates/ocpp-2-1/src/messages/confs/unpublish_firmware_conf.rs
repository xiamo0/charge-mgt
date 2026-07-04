//! UnpublishFirmware Confirmation (Block L)
use serde::{Deserialize, Serialize};
use crate::common::{UnpublishFirmwareStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareConfirmation {
    pub status: UnpublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
