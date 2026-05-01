//! TriggerMessage 消息及处理器

use super::super::confs::trigger_message_conf::TriggerMessageConfirmation;
use crate::common::status::MessageTrigger;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerMessageRequest {
    pub requested_message: MessageTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

pub trait TriggerMessageHandler: Send + Sync {
    fn handle(&self, req: TriggerMessageRequest) -> TriggerMessageConfirmation;
}

pub struct DefaultTriggerMessageHandler;

impl Default for DefaultTriggerMessageHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultTriggerMessageHandler {
    pub fn new() -> Self {
        Self
    }
}

impl TriggerMessageHandler for DefaultTriggerMessageHandler {
    fn handle(&self, _req: TriggerMessageRequest) -> TriggerMessageConfirmation {
        TriggerMessageConfirmation::accepted()
    }
}
