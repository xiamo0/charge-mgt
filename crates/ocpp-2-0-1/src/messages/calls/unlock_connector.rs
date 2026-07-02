//! UnlockConnector Request (Functional Block F)
//! 远程解锁连接器

use serde::{Deserialize, Serialize};

/// UnlockConnector 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    /// EVSE ID (>0)
    pub evse_id: i32,
    /// 连接器 ID (>0)
    pub connector_id: i32,
}

impl UnlockConnectorRequest {
    pub fn new(evse_id: i32, connector_id: i32) -> Self {
        Self {
            evse_id,
            connector_id,
        }
    }
}

pub const ACTION: &str = "UnlockConnector";