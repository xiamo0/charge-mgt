//! PullDynamicScheduleUpdate Confirmation (Block K — 2.1 New)
use crate::common::{ChargingProfileStatusEnumType, ChargingScheduleUpdateType, StatusInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateConfirmation {
    pub status: ChargingProfileStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_update: Option<ChargingScheduleUpdateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
