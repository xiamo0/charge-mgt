//! UnpublishFirmware Confirmation (Block L)
use crate::common::{StatusInfoType, UnpublishFirmwareStatusEnumType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnpublishFirmwareConfirmation {
    pub status: UnpublishFirmwareStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
