//! MeterValues 消息及处理器
//!
//! 定义充电点上报的电表样本（MeterValues）请求和处理器接口，以及默认实现。

use super::super::confs::meter_values_conf::MeterValuesConfirmation;
use crate::common::meter_value::MeterValue;
use serde::{Deserialize, Serialize};

/// MeterValues 请求，包含连接器编号、可选事务 ID 与一组 MeterValue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeterValuesRequest {
    /// 连接器���号
    pub connector_id: i32,
    /// 可选的事务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    /// 本次上报的计量值数组
    pub meter_value: Vec<MeterValue>,
}

/// MeterValues 处理器 trait，接收请求并返回确认
pub trait MeterValuesHandler: Send + Sync {
    fn handle(&self, req: MeterValuesRequest) -> MeterValuesConfirmation;
}

/// 默认的 MeterValues 处理器（空实现）
pub struct DefaultMeterValuesHandler;

impl Default for DefaultMeterValuesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultMeterValuesHandler {
    /// 创建默认处理器实例
    pub fn new() -> Self {
        Self
    }
}

impl MeterValuesHandler for DefaultMeterValuesHandler {
    fn handle(&self, _req: MeterValuesRequest) -> MeterValuesConfirmation {
        MeterValuesConfirmation
    }
}
