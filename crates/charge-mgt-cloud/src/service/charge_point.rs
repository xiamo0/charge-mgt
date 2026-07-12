//! 充电桩业务逻辑。

use chrono::Local;
use sea_orm::*;

use crate::dto::charge_point::{
    ChargePointListQuery, ChargePointResponse, CreateChargePoint, UpdateChargePoint,
};
use crate::dto::common::PageResult;
use crate::entity::charge_point::{ActiveModel, Column, Entity, Model};
use crate::error::AppError;

/// 列表分页查询。
///
/// 筛选逻辑：
/// * `station_id` / `status` 可选
/// * `include_deleted == false`（默认）时过滤 `is_deleted = 0`
///
/// **错误**：`Db`（DB 错误）。
pub async fn list(
    db: &DatabaseConnection,
    q: ChargePointListQuery,
) -> Result<PageResult<ChargePointResponse>, AppError> {
    let page = q.page_query();
    let mut select = Entity::find();
    if let Some(sid) = q.station_id {
        select = select.filter(Column::StationId.eq(sid));
    }
    if let Some(status) = &q.status {
        select = select.filter(Column::Status.eq(status.clone()));
    }
    if !q.include_deleted.unwrap_or(false) {
        select = select.filter(Column::IsDeleted.eq(0_i16));
    }
    let paginator = select.paginate(db, page.page_size);
    let total = paginator.num_items().await?;
    let page_idx = page.page.saturating_sub(1);
    let items = paginator.fetch_page(page_idx).await?;
    Ok(PageResult {
        items,
        total,
        page: page.page,
        page_size: page.page_size,
    })
}

/// 按 `charge_point_id` 取详情，**不**过滤软删除。
///
/// **错误**：`NotFound`（不存在） / `Db`。
pub async fn get(db: &DatabaseConnection, id: &str) -> Result<Model, AppError> {
    Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("charge point {id}")))
}

/// 创建充电桩。
///
/// **副作用**：自动写入 `is_deleted = 0` / `create_time` / `update_time`。
///
/// **错误**：
/// * `Conflict`：`charge_point_id` 已存在
/// * `Db`：DB 错误
pub async fn create(db: &DatabaseConnection, req: CreateChargePoint) -> Result<Model, AppError> {
    // 业务级唯一性校验（DB 没有 UNIQUE 索引，依赖 charge_point_id 作为主键
    // 已经隐式保证）；此处显式查一次，返回 409 而非依赖 sea-orm 的 DbErr
    if Entity::find_by_id(req.charge_point_id.clone())
        .one(db)
        .await?
        .is_some()
    {
        return Err(AppError::conflict(format!(
            "charge point {} already exists",
            req.charge_point_id
        )));
    }
    let now = Local::now().naive_local();
    let model = ActiveModel {
        charge_point_id: Set(req.charge_point_id),
        station_id: Set(req.station_id),
        charge_point_vendor: Set(req.charge_point_vendor),
        charge_point_model: Set(req.charge_point_model),
        charge_box_serial_number: Set(req.charge_box_serial_number),
        charge_point_serial_number: Set(req.charge_point_serial_number),
        firmware_version: Set(req.firmware_version),
        iccid: Set(req.iccid),
        imsi: Set(req.imsi),
        meter_type: Set(req.meter_type),
        meter_serial_number: Set(req.meter_serial_number),
        status: Set(req.status),
        error_code: Set(req.error_code),
        install_date: Set(req.install_date),
        is_deleted: Set(0),
        create_time: Set(now),
        update_time: Set(now),
    };
    let res = model.insert(db).await?;
    Ok(res)
}

/// 部分更新（PATCH）。
///
/// `Option` 为 `None` 的字段**保持原值不变**；自动刷新 `update_time`。
///
/// **错误**：`NotFound` / `Db`。
pub async fn update(
    db: &DatabaseConnection,
    id: &str,
    req: UpdateChargePoint,
) -> Result<Model, AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    if let Some(v) = req.station_id {
        active.station_id = Set(v);
    }
    if let Some(v) = req.charge_point_vendor {
        active.charge_point_vendor = Set(Some(v));
    }
    if let Some(v) = req.charge_point_model {
        active.charge_point_model = Set(Some(v));
    }
    if let Some(v) = req.charge_box_serial_number {
        active.charge_box_serial_number = Set(Some(v));
    }
    if let Some(v) = req.charge_point_serial_number {
        active.charge_point_serial_number = Set(Some(v));
    }
    if let Some(v) = req.firmware_version {
        active.firmware_version = Set(Some(v));
    }
    if let Some(v) = req.iccid {
        active.iccid = Set(Some(v));
    }
    if let Some(v) = req.imsi {
        active.imsi = Set(Some(v));
    }
    if let Some(v) = req.meter_type {
        active.meter_type = Set(Some(v));
    }
    if let Some(v) = req.meter_serial_number {
        active.meter_serial_number = Set(Some(v));
    }
    if let Some(v) = req.status {
        active.status = Set(v);
    }
    if let Some(v) = req.error_code {
        active.error_code = Set(Some(v));
    }
    if let Some(v) = req.install_date {
        active.install_date = Set(Some(v));
    }
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}

/// 软删除：置 `is_deleted = 1`，**不**物理删除。
///
/// **错误**：`NotFound` / `Db`。
pub async fn soft_delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
    let existing = get(db, id).await?;
    let mut active: ActiveModel = existing.into();
    active.is_deleted = Set(1);
    active.update_time = Set(Local::now().naive_local());
    active.update(db).await?;
    Ok(())
}

/// 恢复软删除：置 `is_deleted = 0`。
///
/// 即使记录当前不是软删除状态，调用也成功（幂等）。
///
/// **错误**：`NotFound` / `Db`。
pub async fn restore(db: &DatabaseConnection, id: &str) -> Result<Model, AppError> {
    let existing = Entity::find_by_id(id.to_owned())
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("charge point {id}")))?;
    let mut active: ActiveModel = existing.into();
    active.is_deleted = Set(0);
    active.update_time = Set(Local::now().naive_local());
    Ok(active.update(db).await?)
}
