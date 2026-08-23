//! 角色权限检查 helper
//!
//! - `require_write_access`：Admin 或 Operator 允许；Viewer 拒绝
//! - `require_delete_access`：仅 Admin 允许
//!
//! 失败返 `AppError::Forbidden`（HTTP 403）。
//! 不区分具体角色，避免泄露「admin 才行 / operator 不行」等枚举信息。

use crate::auth::middleware::AuthContext;
use crate::error::AppError;
use crate::ocpp16::entity::operator::Role;

/// 写操作权限：Admin / Operator 通过；Viewer 拒绝
pub fn require_write_access(ctx: &AuthContext) -> Result<(), AppError> {
    if ctx.role.can_write() {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "当前角色无写权限".into(),
        ))
    }
}

/// 删除操作权限：仅 Admin 通过
pub fn require_delete_access(ctx: &AuthContext) -> Result<(), AppError> {
    if ctx.role.can_delete() {
        Ok(())
    } else {
        Err(AppError::Forbidden(
            "当前角色无删除权限".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocpp16::entity::operator::Role;

    fn ctx(role: Role) -> AuthContext {
        AuthContext {
            operator_id: 1,
            username: "test".into(),
            role,
        }
    }

    #[test]
    fn admin_can_write_and_delete() {
        assert!(require_write_access(&ctx(Role::Admin)).is_ok());
        assert!(require_delete_access(&ctx(Role::Admin)).is_ok());
    }

    #[test]
    fn operator_can_write_but_not_delete() {
        assert!(require_write_access(&ctx(Role::Operator)).is_ok());
        assert!(require_delete_access(&ctx(Role::Operator)).is_err());
    }

    #[test]
    fn viewer_cannot_write_or_delete() {
        assert!(require_write_access(&ctx(Role::Viewer)).is_err());
        assert!(require_delete_access(&ctx(Role::Viewer)).is_err());
    }
}
