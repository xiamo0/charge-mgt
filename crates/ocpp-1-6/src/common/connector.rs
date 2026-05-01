//! 连接器类型

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connector {
    pub id: i32,
    pub status: super::status::ChargePointStatus,
    pub error_code: super::status::ChargePointErrorCode,
}