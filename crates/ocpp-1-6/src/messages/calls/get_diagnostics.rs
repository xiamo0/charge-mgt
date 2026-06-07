//! GetDiagnostics 消息及处理器
//!
//! 请求服务器准备诊断数据（如日志或支持包），包含目标位置与可选的时间窗口与重试策略。

use super::super::confs::get_diagnostics_conf::GetDiagnosticsConfirmation;
use serde::{Deserialize, Serialize};

/// GetDiagnostics 请求，包含下载位置、可选重试信息与时间窗口
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDiagnosticsRequest {
    /// 服务器接收或上传诊断包的位置（URL 或路径）
    pub location: String,
    /// 可选重试次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// 可选重试间隔（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    /// 可选的开始时间（RFC3339），用于限制诊断数据范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 可选的结束时间（RFC3339）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<String>,
}

/// GetDiagnostics 处理器接口
pub trait GetDiagnosticsHandler: Send + Sync {
    fn handle(&self, req: GetDiagnosticsRequest) -> GetDiagnosticsConfirmation;
}

/// 默认实现：返回空确认
pub struct DefaultGetDiagnosticsHandler;

impl Default for DefaultGetDiagnosticsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultGetDiagnosticsHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl GetDiagnosticsHandler for DefaultGetDiagnosticsHandler {
    fn handle(&self, _req: GetDiagnosticsRequest) -> GetDiagnosticsConfirmation {
        GetDiagnosticsConfirmation::empty()
    }
}
