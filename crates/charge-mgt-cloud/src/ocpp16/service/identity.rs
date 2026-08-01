//! 身份标签业务逻辑。
//!
//! **删除语义**：业务上 DELETE 不物理删除，而是将 `status` 置为
//! [`IdentityStatus::Blocked`]（挂失），由 [`to_blocked_status`] 实现。
//! 这与 [`crate::service::charge_point::soft_delete`] 的 `is_deleted` 标志不同。

use chrono::Local;
use sea_orm::*;

use crate::error::AppError;
use crate::ocpp16::dto::common::PageResult;
use crate::ocpp16::dto::identity_info::{
    CreateIdentity, IdentityListQuery, IdentityResponse, UpdateIdentity,
};
use crate::ocpp16::entity::enums::IdentityStatus;
use crate::ocpp16::entity::identity_info::{ActiveModel, Column, Entity, Model};

/// 列表分页查询；可选过滤 `user_id` / `tag_type` / `status`。
///
/// **错误**：`Db`。
pub async fn list(
    db: &DatabaseConnection,
    q: IdentityListQuery,
) -> Result<PageResult<IdentityResponse>, AppError> {
    let page = q.page_query();
    let mut select = Entity::find();
    if let Some(uid) = q.user_id {
        select = select.filter(Column::UserId.eq(uid));
    }
    if let Some(tt) = q.tag_type {
        select = select.filter(Column::TagType.eq(tt));
    }
    if let Some(s) = q.status {
        select = select.filter(Column::Status.eq(s));
    }
    let paginator = select.paginate(db, page.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(page.page.saturating_sub(1)).await?;
    Ok(PageResult {
        items,
        total,
        page: page.page,
        page_size: page.page_size,
    })
}

/// 按主键 `id` 取详情。
///
/// **错误**：`NotFound` / `Db`。
pub async fn get(db: &DatabaseConnection, id: i64) -> Result<Model, AppError> {
    Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("identity {id}")))
}

/// 按 `tag_id`（UNIQUE 索引列）查。
///
/// 主要用于 OCPP Authorize 流程（外部以 tag_id 而非自增 id 查询）。
///
/// **错误**：`NotFound` / `Db`。
pub async fn get_by_tag(db: &DatabaseConnection, tag_id: &str) -> Result<Model, AppError> {
    Entity::find()
        .filter(Column::TagId.eq(tag_id.to_owned()))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("identity with tag {tag_id}")))
}

/// 创建标签。
///
/// **错误**：
/// * `Conflict`：`tag_id` 已存在
/// * `Db`：DB 错误
pub async fn create(db: &DatabaseConnection, req: CreateIdentity) -> Result<Model, AppError> {
    if Entity::find()
        .filter(Column::TagId.eq(req.tag_id.clone()))
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::conflict(format!(
            "tag_id {} already exists",
            req.tag_id
        )));
    }
    let now = Local::now().naive_local();
    let model = ActiveModel {
        id: Default::default(),
        user_id: Set(req.user_id),
        tag_id: Set(req.tag_id),
        tag_type: Set(req.tag_type),
        status: Set(req.status),
        expire_time: Set(req.expire_time),
        create_time: Set(now),
        update_time: Set(now),
    };
    Ok(model.insert(db).await?)
}

/// 部分更新；自动刷新 `update_time`。
///
/// **错误**：`NotFound` / `Db`。
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    req: UpdateIdentity,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    if let Some(v) = req.user_id {
        active.user_id = Set(Some(v));
    }
    if let Some(v) = req.tag_type {
        active.tag_type = Set(v);
    }
    if let Some(v) = req.status {
        active.status = Set(v);
    }
    if let Some(v) = req.expire_time {
        active.expire_time = Set(Some(v));
    }
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}

/// 把身份状态置为 [`IdentityStatus::Blocked`]（挂失）。
///
/// **注意**：这是 DELETE HTTP 端点的实现，等价于"软删除"，**不**物理删除。
/// 保留所有审计轨迹。
///
/// **错误**：`NotFound` / `Db`。
pub async fn to_blocked_status(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    active.status = Set(IdentityStatus::Blocked);
    active.update_time = Set(Local::now().naive_local());
    active.update(db).await?;
    Ok(())
}

/// 重新激活（Blocked → Accepted）。
///
/// **前置条件**：当前状态不能是 [`IdentityStatus::Expired`]，否则返回 400
/// （过期标签需要走续期流程，不是简单 activate）。
///
/// **错误**：`NotFound` / `BadRequest` / `Db`。
pub async fn activate(db: &DatabaseConnection, id: i64) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    if existing.status == IdentityStatus::Expired {
        return Err(AppError::bad_request("cannot activate expired identity"));
    }
    let mut active: ActiveModel = existing.into();
    active.status = Set(IdentityStatus::Accepted);
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}
