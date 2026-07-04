//! 充电枪资源 DTO。
//!
//! 注意：枪记录由 OCPP StatusNotification 触发创建，**无** CreateConnector
//! 请求体（POST 端点不存在）。Update 即可修改类型/状态/错误码。

use serde::Deserialize;

use crate::entity::enums::ConnectorType;

/// `PATCH /api/v1/charge-points/:pid/connectors/:cid` 请求体。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChargeConnector {
    pub connector_id: Option<String>,
    pub connector_type: Option<ConnectorType>,
    pub status: Option<String>,
    pub error_code: Option<String>,
}

/// `GET /api/v1/connectors` 与嵌套 `GET /api/v1/charge-points/:pid/connectors`
/// 共用的 query string。
#[derive(Debug, Default, Deserialize)]
pub struct ChargeConnectorListQuery {
    /// 按 `charge_point_id` 过滤；嵌套端点强制写入此字段
    #[serde(default)]
    pub charge_point_id: Option<String>,
    /// 按状态字符串过滤
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl ChargeConnectorListQuery {
    /// 转 [`super::common::PageQuery`]。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 充电枪响应体。
pub type ChargeConnectorResponse = crate::entity::charge_connector::Model;
