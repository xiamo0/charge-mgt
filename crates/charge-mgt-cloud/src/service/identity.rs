use chrono::Local;
use sea_orm::*;

use crate::dto::common::PageResult;
use crate::dto::identity_info::{
    CreateIdentity, IdentityListQuery, IdentityResponse, UpdateIdentity,
};
use crate::entity::enums::IdentityStatus;
use crate::entity::identity_info::{ActiveModel, Column, Entity, Model};
use crate::error::AppError;

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

pub async fn get(db: &DatabaseConnection, id: i64) -> Result<Model, AppError> {
    Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("identity {id}")))
}

pub async fn get_by_tag(db: &DatabaseConnection, tag_id: &str) -> Result<Model, AppError> {
    Entity::find()
        .filter(Column::TagId.eq(tag_id.to_owned()))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("identity with tag {tag_id}")))
}

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

pub async fn to_blocked_status(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    active.status = Set(IdentityStatus::Blocked);
    active.update_time = Set(Local::now().naive_local());
    active.update(db).await?;
    Ok(())
}

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
