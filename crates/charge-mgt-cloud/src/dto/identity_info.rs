//! 身份标签资源 DTO。

use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::entity::enums::{IdentityStatus, TagType};

/// `POST /api/v1/identities` 请求体。
///
/// `tag_id` 在 DB 上有 UNIQUE 约束，重复创建返回 409。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateIdentity {
    /// 可空：未绑定用户或共享 Tag 场景
    pub user_id: Option<i64>,
    pub tag_id: String,
    pub tag_type: TagType,
    /// 默认 [`IdentityStatus::Accepted`]（通过 serde default）
    #[serde(default = "default_accepted")]
    pub status: IdentityStatus,
    /// 可选有效期；`None` 表示永久有效
    pub expire_time: Option<NaiveDateTime>,
}

fn default_accepted() -> IdentityStatus {
    IdentityStatus::Accepted
}

/// `PATCH /api/v1/identities/:id` 请求体。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateIdentity {
    pub user_id: Option<i64>,
    pub tag_type: Option<TagType>,
    pub status: Option<IdentityStatus>,
    pub expire_time: Option<NaiveDateTime>,
}

/// `GET /api/v1/identities` query string。
#[derive(Debug, Default, Deserialize)]
pub struct IdentityListQuery {
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub tag_type: Option<TagType>,
    #[serde(default)]
    pub status: Option<IdentityStatus>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
}

impl IdentityListQuery {
    /// 转 [`super::common::PageQuery`]。
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

/// 身份标签响应体。
pub type IdentityResponse = crate::entity::identity_info::Model;
