//! ReserveNow Request (Functional Block H)
//! 创建预约

use crate::common::IdTokenType;
use serde::{Deserialize, Serialize};

/// 连接器类型枚举
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectorEnumType {
    CCCS1,
    CCCS2,
    CGBT,
    CType1,
    CType2,
    SType2,
    // ... 其他类型
}

/// ReserveNow 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    /// 预约 ID
    pub id: i32,
    /// 过期时间
    pub expiry_date_time: String,
    /// ID Token
    pub id_token: IdTokenType,
    /// 连接器类型 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorEnumType>,
    /// EVSE ID (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// 组 ID Token (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

impl ReserveNowRequest {
    pub fn new(id: i32, expiry_date_time: impl Into<String>, id_token: IdTokenType) -> Self {
        Self {
            id,
            expiry_date_time: expiry_date_time.into(),
            id_token,
            connector_type: None,
            evse_id: None,
            group_id_token: None,
        }
    }

    /// 设置连接器类型
    pub fn with_connector_type(mut self, connector_type: ConnectorEnumType) -> Self {
        self.connector_type = Some(connector_type);
        self
    }

    /// 设置 EVSE ID
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }

    /// 设置组 ID Token
    pub fn with_group_id_token(mut self, group_id_token: IdTokenType) -> Self {
        self.group_id_token = Some(group_id_token);
        self
    }
}

pub const ACTION: &str = "ReserveNow";
