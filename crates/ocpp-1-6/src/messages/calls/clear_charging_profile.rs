//! ClearChargingProfile 消息及处理器
//!
//! 清除充电档案的请求和处理器定义。

use super::super::confs::clear_charging_profile_conf::ClearChargingProfileConfirmation;
use crate::common::status::ChargingProfilePurpose;
use serde::{Deserialize, Serialize};

/// ClearChargingProfile 请求，可选择性指定连接器、charging profile purpose 与 stack level
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClearChargingProfileRequest {
    /// 可选的连接器编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    /// 可选的 charging profile purpose（如 TxProfile）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurpose>,
    /// 可选的堆栈级别（stackLevel）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

/// ClearChargingProfile 处理器接口
pub trait ClearChargingProfileHandler: Send + Sync {
    fn handle(&self, req: ClearChargingProfileRequest) -> ClearChargingProfileConfirmation;
}

/// 默认实现：直接接受清除请求
pub struct DefaultClearChargingProfileHandler;

impl Default for DefaultClearChargingProfileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultClearChargingProfileHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl ClearChargingProfileHandler for DefaultClearChargingProfileHandler {
    fn handle(&self, _req: ClearChargingProfileRequest) -> ClearChargingProfileConfirmation {
        ClearChargingProfileConfirmation::accepted()
    }
}
