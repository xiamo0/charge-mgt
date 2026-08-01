//! 用户身份标签/鉴权 entity（对应 `identity_info` 表）。
//!
//! 鉴权流程入口：OCPP Authorize 报文以 `tag_id` 查本表，得到 `status` 后回写
//! `Accepted` / `Blocked` 给桩端。注意 `tag_id` 列在 DB 上有 UNIQUE 约束，
//! 重复创建会被 DB 拒绝（service 层预先查询以返回业务级 409）。

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::enums::{IdentityStatus, TagType};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "identity_info")]
pub struct Model {
    /// 主键（自增）
    #[sea_orm(primary_key)]
    pub id: i64,
    /// 关联的用户 ID（可为空，支持未绑定或共享 Tag）
    pub user_id: Option<i64>,
    /// 标签值（RFID 卡号、虚拟 Token 字符串等），有 UNIQUE 索引
    pub tag_id: String,
    /// 标签类型：1 RFID、2 二维码、3 车牌号、4 App 虚拟 Token
    #[sea_orm(column_type = "SmallInteger")]
    pub tag_type: TagType,
    /// 鉴权状态：1 有效、2 挂失、3 已过期
    #[sea_orm(column_type = "SmallInteger")]
    pub status: IdentityStatus,
    /// 标签有效期（用于临时卡或月卡管理）
    #[sea_orm(column_type = "Timestamp")]
    pub expire_time: Option<DateTime>,
    /// 记录创建时间
    #[sea_orm(column_type = "Timestamp")]
    pub create_time: DateTime,
    /// 记录更新时间
    #[sea_orm(column_type = "Timestamp")]
    pub update_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
