use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::entity::enums::{IdentityStatus, TagType};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateIdentity {
    pub user_id: Option<i64>,
    pub tag_id: String,
    pub tag_type: TagType,
    #[serde(default = "default_accepted")]
    pub status: IdentityStatus,
    pub expire_time: Option<NaiveDateTime>,
}

fn default_accepted() -> IdentityStatus {
    IdentityStatus::Accepted
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateIdentity {
    pub user_id: Option<i64>,
    pub tag_type: Option<TagType>,
    pub status: Option<IdentityStatus>,
    pub expire_time: Option<NaiveDateTime>,
}

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
    pub fn page_query(&self) -> super::common::PageQuery {
        super::common::PageQuery {
            page: self.page.unwrap_or(1),
            page_size: self.page_size.unwrap_or(20),
        }
        .normalize()
    }
}

pub type IdentityResponse = crate::entity::identity_info::Model;
