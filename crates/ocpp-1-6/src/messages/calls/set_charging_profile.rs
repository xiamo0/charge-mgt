//! SetChargingProfile 消息及处理器
//!
//! 设置充电档案的请求与处理器，允许后台下发充电策略到充电点。

use super::super::confs::set_charging_profile_conf::SetChargingProfileConfirmation;
use crate::common::configuration::ChargingProfile;
use serde::{Deserialize, Serialize};

/// SetChargingProfile 请求，包含目标连接器与完整的 ChargingProfile
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetChargingProfileRequest {
    /// 目标连接器编号
    pub connector_id: i32,
    /// 要下发的充电档案
    pub charging_profile: ChargingProfile,
}

/// SetChargingProfile 处理器接口
pub trait SetChargingProfileHandler: Send + Sync {
    fn handle(&self, req: SetChargingProfileRequest) -> SetChargingProfileConfirmation;
}

/// 默认实现：接受配置
pub struct DefaultSetChargingProfileHandler;

impl Default for DefaultSetChargingProfileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSetChargingProfileHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl SetChargingProfileHandler for DefaultSetChargingProfileHandler {
    fn handle(&self, _req: SetChargingProfileRequest) -> SetChargingProfileConfirmation {
        SetChargingProfileConfirmation::accepted()
    }
}
