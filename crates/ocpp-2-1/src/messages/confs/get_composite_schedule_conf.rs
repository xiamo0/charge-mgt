//! GetCompositeSchedule Confirmation (Block K)
use serde::{Deserialize, Serialize};
use crate::common::{CompositeScheduleType, GenericStatusEnumType, StatusInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleConfirmation {
    pub status: GenericStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CompositeScheduleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
