//! GetCompositeSchedule 响应

use crate::common::configuration::ChargingSchedule;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompositeScheduleConfirmation {
    pub status: crate::common::status::GetCompositeScheduleStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ChargingSchedule>,
}

impl GetCompositeScheduleConfirmation {
    pub fn empty() -> Self {
        Self {
            status: crate::common::status::GetCompositeScheduleStatus::Accepted,
            schedule_start: None,
            schedule: None,
        }
    }
}
