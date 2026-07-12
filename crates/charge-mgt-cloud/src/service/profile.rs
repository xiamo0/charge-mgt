//! 智能充电策略业务逻辑。

use chrono::Local;
use sea_orm::*;

use crate::dto::common::PageResult;
use crate::dto::smart_charge_profile::{CreateProfile, ProfileListQuery, ProfileResponse};
use crate::entity::smart_charge_profile::{ActiveModel, Column, Entity, Model};
use crate::error::AppError;

/// 列表分页查询，可选过滤 `charge_point_id` / `connector_id` /
/// `charging_profile_purpose` / `status`。
///
/// **错误**：`Db`。
pub async fn list(
    db: &DatabaseConnection,
    q: ProfileListQuery,
) -> Result<PageResult<ProfileResponse>, AppError> {
    let page = q.page_query();
    let mut select = Entity::find();
    if let Some(pid) = &q.charge_point_id {
        select = select.filter(Column::ChargePointId.eq(pid.clone()));
    }
    if let Some(cid) = &q.connector_id {
        select = select.filter(Column::ConnectorId.eq(cid.clone()));
    }
    if let Some(pur) = &q.charging_profile_purpose {
        select = select.filter(Column::ChargingProfilePurpose.eq(pur.clone()));
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
        .ok_or_else(|| AppError::not_found(format!("profile {id}")))
}

/// 按 `charge_point_id` 列出该桩下所有策略（**不**分页）。
///
/// 供嵌套端点 `GET /api/v1/charge-points/:pid/charging-profiles` 使用。
///
/// **错误**：`Db`。
pub async fn list_by_charge_point(
    db: &DatabaseConnection,
    charge_point_id: &str,
) -> Result<Vec<Model>, AppError> {
    let items = Entity::find()
        .filter(Column::ChargePointId.eq(charge_point_id.to_owned()))
        .all(db)
        .await?;
    Ok(items)
}

/// 创建策略。
///
/// **副作用**：DB INSERT 后会触发 OCPP `SetChargingProfile` 下发（由 caller
/// 负责）；本函数只写 DB，下发逻辑不在 service 层。
///
/// **错误**：`Db`。
pub async fn create(db: &DatabaseConnection, req: CreateProfile) -> Result<Model, AppError> {
    let now = Local::now().naive_local();
    let model = ActiveModel {
        id: Default::default(),
        charge_point_id: Set(req.charge_point_id),
        connector_id: Set(req.connector_id),
        charging_profile_id: Set(req.charging_profile_id),
        stack_level: Set(req.stack_level),
        charging_profile_purpose: Set(req.charging_profile_purpose),
        charging_profile_kind: Set(req.charging_profile_kind),
        start_time: Set(req.start_time),
        duration: Set(req.duration),
        max_power_kw: Set(req.max_power_kw),
        max_current_a: Set(req.max_current_a),
        status: Set(req.status),
        create_time: Set(now),
        update_time: Set(now),
    };
    Ok(model.insert(db).await?)
}

/// 物理删除策略。
///
/// **副作用**：DB DELETE 后会触发 OCPP `ClearChargingProfile` 下发（由 caller
/// 负责）。本函数只删 DB 行，0 行影响视为 NotFound。
///
/// **错误**：`NotFound` / `Db`。
pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), AppError> {
    let result = Entity::delete_by_id(id).exec(db).await?;
    if result.rows_affected == 0 {
        return Err(AppError::not_found(format!("profile {id}")));
    }
    Ok(())
}
