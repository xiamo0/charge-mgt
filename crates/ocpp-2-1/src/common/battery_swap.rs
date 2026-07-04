//! OCPP 2.1 BatterySwap types (Functional Block S)

use serde::{Deserialize, Serialize};

/// 电池交换事件类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BatterySwapEventEnumType {
    BatteryIn,
    BatteryOut,
    BatteryOutTimeout,
}

/// 电池数据类型
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryDataType {
    pub evse_id: i32,
    pub serial_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_c: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_h: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_info: Option<String>,
}
