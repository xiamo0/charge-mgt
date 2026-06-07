//! GetCompositeSchedule 消息及处理器
//!
//! 提供查询复合调度（Composite Schedule）的请求类型和处理器接口，默认实现返回空确认。

use super::super::confs::get_composite_schedule_conf::GetCompositeScheduleConfirmation;
use serde::{Deserialize, Serialize};

/// GetCompositeSchedule 请求，包含连接器 ID、查询时长以及可选的充电档案用途（purpose）
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCompositeScheduleRequest {
    /// 连接器编号
    pub connector_id: i32,
    /// 期望的时长（秒）
    pub duration_secs: i32,
    /// 可选的 charging profile purpose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<i32>,
}

/// GetCompositeSchedule 处理器接口
pub trait GetCompositeScheduleHandler: Send + Sync {
    fn handle(&self, req: GetCompositeScheduleRequest) -> GetCompositeScheduleConfirmation;
}

/// 默认实现（返回空确认）
pub struct DefaultGetCompositeScheduleHandler;

impl Default for DefaultGetCompositeScheduleHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetCompositeScheduleHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl GetCompositeScheduleHandler for DefaultGetCompositeScheduleHandler {
    fn handle(&self, _req: GetCompositeScheduleRequest) -> GetCompositeScheduleConfirmation {
        GetCompositeScheduleConfirmation::empty()
    }
}
