//! 运营账号 entity（对应 `mgt_operators` 表）。
//!
//! HTTP 管理 API 的鉴权主体。`role` 字段为 0=admin / 1=operator / 2=viewer。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "mgt_operators")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub username: String,
    /// argon2id 密码哈希
    pub password_hash: String,
    /// 0=admin, 1=operator, 2=viewer
    pub role: i16,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 角色枚举（对应 `role` SMALLINT 列）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i16)]
pub enum Role {
    Admin = 0,
    Operator = 1,
    Viewer = 2,
}

impl Role {
    pub fn from_i16(v: i16) -> Self {
        match v {
            0 => Self::Admin,
            1 => Self::Operator,
            _ => Self::Viewer,
        }
    }

    /// admin 可写可删；operator 可写不可删；viewer 只读
    pub fn can_write(&self) -> bool {
        matches!(self, Self::Admin | Self::Operator)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, Self::Admin)
    }
}
