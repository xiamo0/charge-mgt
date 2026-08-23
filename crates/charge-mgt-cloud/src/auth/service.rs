//! 登录业务逻辑

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::error::AppError;
use crate::ocpp16::entity::operator::{Column, Entity as Operators, Model, Role};
use crate::state::AppState;

use super::jwt;
use super::password;

/// 登录成功返回
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub operator: OperatorInfo,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OperatorInfo {
    pub id: i64,
    pub username: String,
    pub role: i16,
}

/// 登录：查 operator → 验密码 → 发 JWT
pub async fn login(
    state: &AppState,
    username: &str,
    password: &str,
) -> Result<LoginResponse, AppError> {
    let db = state.db()?;
    let secret = &state.config()?.auth.jwt_secret;
    let ttl = state.config()?.auth.access_token_ttl_secs;

    let model = Operators::find()
        .filter(Column::Username.eq(username))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("用户名或密码错误".into()))?;

    if !model.is_active {
        return Err(AppError::Unauthorized("账号已停用".into()));
    }

    if !password::verify_password(&model.password_hash, password)? {
        return Err(AppError::Unauthorized("用户名或密码错误".into()));
    }

    let token = jwt::encode_token(secret, model.id, &model.username, Role::from_i16(model.role), ttl)?;

    Ok(LoginResponse {
        token,
        operator: OperatorInfo {
            id: model.id,
            username: model.username,
            role: model.role,
        },
    })
}

/// 按 id 查 operator（GET /auth/me 用）
pub async fn get_operator(state: &AppState, id: i64) -> Result<Model, AppError> {
    let db = state.db()?;
    Operators::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found(format!("operator {id}")))
}
