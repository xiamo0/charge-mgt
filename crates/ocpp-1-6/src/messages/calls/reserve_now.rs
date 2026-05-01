//! ReserveNow 消息及处理器

use super::super::confs::reserve_now_conf::ReserveNowConfirmation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReserveNowRequest {
    pub connector_id: i32,
    pub expiry_date: String,
    pub id_tag: String,
    pub reservation_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
}

pub trait ReserveNowHandler: Send + Sync {
    fn handle(&self, req: ReserveNowRequest) -> ReserveNowConfirmation;
}

pub struct DefaultReserveNowHandler;

impl Default for DefaultReserveNowHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultReserveNowHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ReserveNowHandler for DefaultReserveNowHandler {
    fn handle(&self, _req: ReserveNowRequest) -> ReserveNowConfirmation {
        ReserveNowConfirmation::accepted()
    }
}
