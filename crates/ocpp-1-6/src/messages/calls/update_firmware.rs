//! UpdateFirmware 消息及处理器
//!
//! 触发固件更新的请求定义与处理器接口。

use super::super::confs::update_firmware_conf::UpdateFirmwareConfirmation;
use serde::{Deserialize, Serialize};

/// UpdateFirmware 请求，包含固件位置、重试策略、请求 ID 与检索时间
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateFirmwareRequest {
    /// 固件文件或服务的 URL
    pub location: String,
    /// 可选重试次数
    pub retries: Option<i32>,
    /// 可选重试间隔（秒）
    pub retry_interval: Option<i32>,
    /// 请求 ID，用于关联固件下载任务
    pub request_id: i32,
    /// 可选的检索时间（RFC3339），服务器指定何时开始下载
    pub retrieve_time: String,
}

/// UpdateFirmware 处理器接口
pub trait UpdateFirmwareHandler: Send + Sync {
    fn handle(&self, req: UpdateFirmwareRequest) -> UpdateFirmwareConfirmation;
}

/// 默认实现：空实现，返回默认确认
pub struct DefaultUpdateFirmwareHandler;

impl Default for DefaultUpdateFirmwareHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultUpdateFirmwareHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl UpdateFirmwareHandler for DefaultUpdateFirmwareHandler {
    fn handle(&self, _req: UpdateFirmwareRequest) -> UpdateFirmwareConfirmation {
        UpdateFirmwareConfirmation
    }
}
