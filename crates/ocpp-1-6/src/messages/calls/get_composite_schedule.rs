//! GetCompositeSchedule 消息及处理器

use super::super::confs::get_composite_schedule_conf::GetCompositeScheduleConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompositeScheduleRequest {
    pub connector_id: i32,
    pub duration_secs: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<i32>,
}

pub trait GetCompositeScheduleHandler: Send + Sync {
    fn handle(&self, req: GetCompositeScheduleRequest) -> GetCompositeScheduleConfirmation;
}

pub struct DefaultGetCompositeScheduleHandler;

impl Default for DefaultGetCompositeScheduleHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetCompositeScheduleHandler {
    pub fn new() -> Self {
        Self
    }
}

impl GetCompositeScheduleHandler for DefaultGetCompositeScheduleHandler {
    fn handle(&self, _req: GetCompositeScheduleRequest) -> GetCompositeScheduleConfirmation {
        GetCompositeScheduleConfirmation::empty()
    }
}
