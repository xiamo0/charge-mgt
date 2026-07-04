//! ChangeAvailability Request (Block G)
use serde::{Deserialize, Serialize};
use crate::common::{EVSEType, OperationalStatusEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    pub operational_status: OperationalStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}

pub const ACTION: &str = "ChangeAvailability";
