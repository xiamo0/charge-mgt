//! 新增 mgt_operators 表：HTTP 管理 API 的运营账号（JWT 鉴权用）

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260118_000001_add_operators"
    }
}

/// 开发用 admin 账号的 argon2id 哈希（密码 "admin123"）
const SEED_ADMIN_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$1WJvLiBPMtjcmen0MsErgg$2mqT/z/2OlUl4YGmLLx8dIftw5gHGwBNvwFspa/KrzI";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS mgt_operators (
                id BIGSERIAL PRIMARY KEY,
                username VARCHAR(64) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                role SMALLINT NOT NULL DEFAULT 1,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .await?;
        db.execute_unprepared(
            "COMMENT ON COLUMN mgt_operators.role IS '0=admin, 1=operator, 2=viewer'",
        )
        .await?;
        // 种子数据：admin/admin123（仅开发）
        db.execute_unprepared(&format!(
            "INSERT INTO mgt_operators (username, password_hash, role) VALUES ('admin', '{}', 0)
             ON CONFLICT (username) DO NOTHING",
            SEED_ADMIN_HASH
        ))
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE IF EXISTS mgt_operators").await?;
        Ok(())
    }
}
