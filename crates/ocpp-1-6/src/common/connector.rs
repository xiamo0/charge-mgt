//! 连接器类型
//!
//! 表示充电点中的单个连接器（connector）状态与错误码信息。
use serde::{Deserialize, Serialize};

/// 单个连接器的状态信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connector {
    /// 连接器编号
    pub id: i32,
    /// 当前连接器的状态（Idle/Charging 等）
    pub status: super::status::ChargePointStatus,
    /// 当前连接器的错误码（若无错误则为 NoError）
    pub error_code: super::status::ChargePointErrorCode,
}
