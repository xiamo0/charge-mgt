//! OCPP 2.1 Periodic Event Stream types (Functional Block N)

use serde::{Deserialize, Serialize};

/// 周期性事件流参数类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicEventStreamParamsType {
    pub interval: f64,
    pub values_per_interval: i32,
}

/// 常量事件流数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstantStreamDataType {
    pub id: i32,
    pub variable_monitoring_id: i32,
    pub params: PeriodicEventStreamParamsType,
}

/// 事件流数据元素
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataElementType {
    pub t: f64,
    pub v: String,
}
