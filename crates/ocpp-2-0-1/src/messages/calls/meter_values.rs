//! MeterValues Request (Functional Block J)
//! 上报电能采样数据（事务外部的周期采样）

use crate::common::MeterValueType;
use serde::{Deserialize, Serialize};

/// MeterValues 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// EVSE ID (>=0, 0=总表)
    pub evse_id: i32,
    /// 计量值列表
    pub meter_value: Vec<MeterValueType>,
}

impl MeterValuesRequest {
    pub fn new(evse_id: i32, meter_value: Vec<MeterValueType>) -> Self {
        Self {
            evse_id,
            meter_value,
        }
    }
}

pub const ACTION: &str = "MeterValues";
