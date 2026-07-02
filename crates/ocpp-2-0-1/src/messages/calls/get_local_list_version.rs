//! GetLocalListVersion Request (Functional Block C)
//! 查询本地白名单版本

use serde::{Deserialize, Serialize};

/// GetLocalListVersion 请求 (空 payload)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLocalListVersionRequest {}

impl GetLocalListVersionRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GetLocalListVersionRequest {
    fn default() -> Self {
        Self::new()
    }
}

pub const ACTION: &str = "GetLocalListVersion";