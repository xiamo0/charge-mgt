//! NotifyDERAlarm Request (Block R — 2.1 New)
use serde::{Deserialize, Serialize};
use crate::common::{DERControlEnumType, GridEventFaultEnumType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERAlarmRequest {
    pub control_type: DERControlEnumType,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_ended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_event_fault: Option<GridEventFaultEnumType>,
}

pub const ACTION: &str = "NotifyDERAlarm";
