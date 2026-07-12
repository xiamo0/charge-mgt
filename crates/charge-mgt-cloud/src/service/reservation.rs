//! 充电预约业务逻辑。
//!
//! **状态机约束**：`update` / `cancel` 仅允许在 `status == Pending` 时调用。
//! 离开 Pending 状态后预约不可再改回（强约束保护 OCPP ReserveNow 协议的
//! 一旦生效不可撤销语义）。

use chrono::Local;
use sea_orm::*;

use crate::dto::charge_reservation::{
    CancelReservation, CreateReservation, ReservationListQuery, ReservationResponse,
    UpdateReservation,
};
use crate::dto::common::PageResult;
use crate::entity::charge_reservation::{ActiveModel, Column, Entity, Model};
use crate::entity::enums::ReservationStatus;
use crate::error::AppError;

/// 列表分页查询，可选过滤 `user_id` / `charge_point_id` / `status`。
///
/// **错误**：`Db`。
pub async fn list(
    db: &DatabaseConnection,
    q: ReservationListQuery,
) -> Result<PageResult<ReservationResponse>, AppError> {
    let page = q.page_query();
    let mut select = Entity::find();
    if let Some(uid) = q.user_id {
        select = select.filter(Column::UserId.eq(uid));
    }
    if let Some(pid) = &q.charge_point_id {
        select = select.filter(Column::ChargePointId.eq(pid.clone()));
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

/// 按主键 `reservation_id` 取详情。
///
/// **错误**：`NotFound` / `Db`。
pub async fn get(db: &DatabaseConnection, id: i64) -> Result<Model, AppError> {
    Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("reservation {id}")))
}

/// 创建预约，初始状态为 [`ReservationStatus::Pending`]。
///
/// **错误**：`BadRequest`（`end_time <= start_time`） / `Db`。
pub async fn create(db: &DatabaseConnection, req: CreateReservation) -> Result<Model, AppError> {
    if req.end_time <= req.start_time {
        return Err(AppError::bad_request("end_time must be after start_time"));
    }
    let now = Local::now().naive_local();
    let model = ActiveModel {
        reservation_id: Default::default(),
        user_id: Set(req.user_id),
        charge_point_id: Set(req.charge_point_id),
        connector_id: Set(req.connector_id),
        tag_id: Set(req.tag_id),
        start_time: Set(req.start_time),
        end_time: Set(req.end_time),
        status: Set(ReservationStatus::Pending),
        transaction_id: Set(None),
        cancel_reason: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
    };
    Ok(model.insert(db).await?)
}

/// 部分更新；仅 `status == Pending` 时允许。
///
/// **错误**：`NotFound` / `BadRequest`（非 Pending 状态）/ `Db`。
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    req: UpdateReservation,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    if existing.status != ReservationStatus::Pending {
        return Err(AppError::bad_request(
            "only pending reservations can be updated",
        ));
    }
    let mut active: ActiveModel = existing.into();
    if let Some(v) = req.connector_id {
        active.connector_id = Set(Some(v));
    }
    if let Some(v) = req.tag_id {
        active.tag_id = Set(Some(v));
    }
    if let Some(v) = req.start_time {
        active.start_time = Set(v);
    }
    if let Some(v) = req.end_time {
        active.end_time = Set(v);
    }
    active.updated_at = Set(Some(Local::now().naive_local()));
    Ok(active.update(db).await?)
}

/// 取消预约；仅 `status == Pending` 时允许，置为 [`ReservationStatus::Cancelled`]。
///
/// **错误**：`NotFound` / `BadRequest`（非 Pending 状态） / `Db`。
pub async fn cancel(
    db: &DatabaseConnection,
    id: i64,
    req: CancelReservation,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    if existing.status != ReservationStatus::Pending {
        return Err(AppError::bad_request(
            "only pending reservations can be cancelled",
        ));
    }
    let mut active: ActiveModel = existing.into();
    active.status = Set(ReservationStatus::Cancelled);
    if let Some(reason) = req.cancel_reason {
        active.cancel_reason = Set(Some(reason));
    }
    active.updated_at = Set(Some(Local::now().naive_local()));
    Ok(active.update(db).await?)
}
