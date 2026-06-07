//! ChangeConfiguration 消息及处理器
//!
//! 用于修改设备配置项的请求与处理器定义。

use super::super::confs::change_configuration_conf::ChangeConfigurationConfirmation;
use serde::{Deserialize, Serialize};

/// 修改配置的请求，包含键和值
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeConfigurationRequest {
    /// 要修改的配置键
    pub key: String,
    /// 新的配置值
    pub value: String,
}

/// ChangeConfiguration 处理器接口
pub trait ChangeConfigurationHandler: Send + Sync {
    fn handle(&self, req: ChangeConfigurationRequest) -> ChangeConfigurationConfirmation;
}

/// 默认实现：直接接受修改
pub struct DefaultChangeConfigurationHandler;

impl Default for DefaultChangeConfigurationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultChangeConfigurationHandler {
    /// 创建默认处理器
    pub fn new() -> Self {
        Self
    }
}

impl ChangeConfigurationHandler for DefaultChangeConfigurationHandler {
    fn handle(&self, _req: ChangeConfigurationRequest) -> ChangeConfigurationConfirmation {
        ChangeConfigurationConfirmation::accepted()
    }
}
