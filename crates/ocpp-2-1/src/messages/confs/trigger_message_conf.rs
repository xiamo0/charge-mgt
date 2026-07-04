//! TriggerMessage Confirmation (Block F)
use serde::{Deserialize, Serialize};
use crate::common::{TriggerMessageStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerMessageConfirmation {
    pub status: TriggerMessageStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
