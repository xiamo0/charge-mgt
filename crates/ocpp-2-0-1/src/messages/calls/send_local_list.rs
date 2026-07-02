//! SendLocalList Request (Functional Block C)
//! 同步本地授权白名单

use serde::{Deserialize, Serialize};
use crate::common::{AuthorizationData, UpdateEnumType};

/// SendLocalList 请求
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    /// 版本号
    pub version_number: i32,
    /// 更新类型
    pub update_type: UpdateEnumType,
    /// 授权列表 (可选)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
}

impl SendLocalListRequest {
    pub fn new(version_number: i32, update_type: UpdateEnumType) -> Self {
        Self {
            version_number,
            update_type,
            local_authorization_list: None,
        }
    }

    /// 添加授权列表
    pub fn with_authorization_list(mut self, list: Vec<AuthorizationData>) -> Self {
        self.local_authorization_list = Some(list);
        self
    }
}

pub const ACTION: &str = "SendLocalList";