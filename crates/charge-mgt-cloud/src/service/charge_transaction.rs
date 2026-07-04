//! 充电事务业务逻辑。
//!
//! **HTTP 入口不开放创建** — 事务由 OCPP StartTransaction 创建。

use chrono::Local;
use sea_orm::*;

use crate::dto::charge_transaction::{
    SettleTransaction, TransactionListQuery, TransactionResponse, UpdateTransaction,
};
use crate::dto::common::PageResult;
use crate::entity::charge_transaction::{ActiveModel, Column, Entity, Model};
use crate::entity::enums::PaymentStatus;
use crate::error::AppError;

/// 列表分页查询，结果按 `start_time` 倒序。
///
/// 筛选条件可选：`user_id` / `charge_point_id` / `status` / `payment_status`
/// / `start_time_from` / `start_time_to` / `include_offline_sync`。
///
/// **错误**：`Db`。
pub async fn list(
    db: &DatabaseConnection,
    q: TransactionListQuery,
) -> Result<PageResult<TransactionResponse>, AppError> {
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
    if let Some(p) = q.payment_status {
        select = select.filter(Column::PaymentStatus.eq(p));
    }
    if let Some(from) = q.start_time_from {
        select = select.filter(Column::StartTime.gte(from));
    }
    if let Some(to) = q.start_time_to {
        select = select.filter(Column::StartTime.lte(to));
    }
    if !q.include_offline_sync.unwrap_or(false) {
        select = select.filter(Column::IsOfflineSync.eq(0_i16));
    }
    let paginator = select.order_by_desc(Column::StartTime).paginate(db, page.page_size);
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
        .ok_or_else(|| AppError::not_found(format!("transaction {id}")))
}

/// 按 `transaction_id`（桩端生成业务键，UNIQUE 索引）取详情。
///
/// 主要用于 OCPP StopTransaction 响应回调与外部系统对账。
///
/// **错误**：`NotFound` / `Db`。
pub async fn get_by_transaction_id(
    db: &DatabaseConnection,
    transaction_id: &str,
) -> Result<Model, AppError> {
    Entity::find()
        .filter(Column::TransactionId.eq(transaction_id.to_owned()))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("transaction {transaction_id}")))
}

/// 部分更新；自动刷新 `update_time`。
///
/// **错误**：`NotFound` / `Db`。
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    req: UpdateTransaction,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    if let Some(v) = req.status {
        active.status = Set(v);
    }
    if let Some(v) = req.stop_reason {
        active.stop_reason = Set(Some(v));
    }
    if let Some(v) = req.end_time {
        active.end_time = Set(Some(v));
    }
    if let Some(v) = req.meter_stop {
        active.meter_stop = Set(Some(v));
    }
    if let Some(v) = req.total_energy {
        active.total_energy = Set(Some(v));
    }
    if let Some(v) = req.payment_status {
        active.payment_status = Set(v);
    }
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}

/// 计费结算：写入金额字段。
///
/// `payment_status` 不提供时默认 [`PaymentStatus::Unpaid`]，
/// 等待第三方支付回调后通过 [`update`] 改为 `Paid`。
///
/// **错误**：`NotFound` / `Db`。
pub async fn settle(
    db: &DatabaseConnection,
    id: i64,
    req: SettleTransaction,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    active.total_amount = Set(Some(req.total_amount));
    active.electricity_fee = Set(Some(req.electricity_fee));
    active.service_fee = Set(Some(req.service_fee));
    active.payment_status = Set(req.payment_status.unwrap_or(PaymentStatus::Unpaid));
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}

/// 退款：仅当 `payment_status == Paid` 时可执行，改为 `Refunded`。
///
/// **错误**：`NotFound` / `BadRequest`（未支付无法退款） / `Db`。
pub async fn refund(db: &DatabaseConnection, id: i64) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    if existing.payment_status != PaymentStatus::Paid {
        return Err(AppError::bad_request("only paid transactions can be refunded"));
    }
    let mut active: ActiveModel = existing.into();
    active.payment_status = Set(PaymentStatus::Refunded);
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}
