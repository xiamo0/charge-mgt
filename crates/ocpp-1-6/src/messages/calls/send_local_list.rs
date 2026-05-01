//! SendLocalList 消息及处理器

use serde::{Deserialize, Serialize};
use super::super::confs::send_local_list_conf::SendLocalListConfirmation;
use crate::common::authorization_list::AuthorizationList;
use crate::common::status::UpdateType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendLocalListRequest {
    pub list_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationList>>,
    #[serde(rename = "updateType")]
    pub update_type: UpdateType,
}

pub trait SendLocalListHandler: Send + Sync {
    fn handle(&self, req: SendLocalListRequest) -> SendLocalListConfirmation;
}

pub struct DefaultSendLocalListHandler;

impl Default for DefaultSendLocalListHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSendLocalListHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SendLocalListHandler for DefaultSendLocalListHandler {
    fn handle(&self, _req: SendLocalListRequest) -> SendLocalListConfirmation {
        SendLocalListConfirmation::accepted()
    }
}