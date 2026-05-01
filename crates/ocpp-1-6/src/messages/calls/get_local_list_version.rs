//! GetLocalListVersion 消息及处理器

use super::super::confs::get_local_list_version_conf::GetLocalListVersionConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetLocalListVersionRequest;

impl GetLocalListVersionRequest {
    pub fn new() -> Self {
        Self
    }
}

pub trait GetLocalListVersionHandler: Send + Sync {
    fn handle(&self, req: GetLocalListVersionRequest) -> GetLocalListVersionConfirmation;
}

pub struct DefaultGetLocalListVersionHandler;

impl Default for DefaultGetLocalListVersionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetLocalListVersionHandler {
    pub fn new() -> Self {
        Self
    }
}

impl GetLocalListVersionHandler for DefaultGetLocalListVersionHandler {
    fn handle(&self, _req: GetLocalListVersionRequest) -> GetLocalListVersionConfirmation {
        GetLocalListVersionConfirmation::new(0)
    }
}
