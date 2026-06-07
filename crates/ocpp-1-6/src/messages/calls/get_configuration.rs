//! GetConfiguration 消息及处理器
//!
//! 定义获取设备配置的请求结构与处理器接口，默认实现返回空配置。

use super::super::confs::get_configuration_conf::GetConfigurationConfirmation;
use serde::{Deserialize, Serialize};

/// GetConfiguration 请求，可选携带需要查询的键列表
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetConfigurationRequest {
    /// 可选的键列表，若为空表示查��所有配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

/// GetConfiguration 处理器接口
pub trait GetConfigurationHandler: Send + Sync {
    fn handle(&self, req: GetConfigurationRequest) -> GetConfigurationConfirmation;
}

/// 默认实现（返回空配置）
pub struct DefaultGetConfigurationHandler;

impl Default for DefaultGetConfigurationHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetConfigurationHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl GetConfigurationHandler for DefaultGetConfigurationHandler {
    fn handle(&self, _req: GetConfigurationRequest) -> GetConfigurationConfirmation {
        GetConfigurationConfirmation::empty()
    }
}
