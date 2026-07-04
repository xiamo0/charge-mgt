use chrono::Local;
use sea_orm::*;

use crate::dto::charge_connector::{
    ChargeConnectorListQuery, ChargeConnectorResponse, UpdateChargeConnector,
};
use crate::dto::common::PageResult;
use crate::entity::charge_connector::{ActiveModel, Column, Entity, Model};
use crate::error::AppError;

pub async fn list(
    db: &DatabaseConnection,
    q: ChargeConnectorListQuery,
) -> Result<PageResult<ChargeConnectorResponse>, AppError> {
    let page = q.page_query();
    let mut select = Entity::find();
    if let Some(pid) = &q.charge_point_id {
        select = select.filter(Column::ChargePointId.eq(pid.clone()));
    }
    if let Some(st) = &q.status {
        select = select.filter(Column::Status.eq(st.clone()));
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

pub async fn get(
    db: &DatabaseConnection,
    charge_point_id: &str,
    connector_id: &str,
) -> Result<Model, AppError> {
    Entity::find_by_id((charge_point_id.to_owned(), connector_id.to_owned()))
        .one(db)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "connector {charge_point_id}/{connector_id}"
            ))
        })
}

pub async fn update(
    db: &DatabaseConnection,
    charge_point_id: &str,
    connector_id: &str,
    req: UpdateChargeConnector,
) -> Result<Model, AppError> {
    let existing = get(db, charge_point_id, connector_id).await?;
    let mut active: ActiveModel = existing.into();
    if let Some(v) = req.connector_type {
        active.connector_type = Set(v);
    }
    if let Some(v) = req.status {
        active.status = Set(v);
    }
    if let Some(v) = req.error_code {
        active.error_code = Set(Some(v));
    }
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}
