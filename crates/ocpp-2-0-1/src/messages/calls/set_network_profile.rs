//! SetNetworkProfile Request (Functional Block B)
//! 设置网络配置

use crate::common::NetworkConnectionProfileType;
use serde::{Deserialize, Serialize};

/// SetNetworkProfile 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    /// 配置 ID
    pub configuration_slot: i32,
    /// 连接配置
    pub connection_data: NetworkConnectionProfileType,
}

impl SetNetworkProfileRequest {
    pub fn new(configuration_slot: i32, connection_data: NetworkConnectionProfileType) -> Self {
        Self {
            configuration_slot,
            connection_data,
        }
    }
}

pub const ACTION: &str = "SetNetworkProfile";
