//! 为 charge_point 表增加 password_hash 列（Basic Auth 模式使用）

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260118_000000_add_charge_point_password"
    }
}

/// 开发用 CP001 的 argon2id 哈希（密码 "test1234"）
const SEED_CP001_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$+FB1WSbWJSdwryazpgErzw$94w8T2u+9QhrshvpZ7EYMtvhEJoxv5R1Gwq0IQkAisY";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE charge_point_ocpp16 ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255) NULL"
        ).await?;
        db.execute_unprepared(
            "COMMENT ON COLUMN charge_point_ocpp16.password_hash IS 'argon2id 密码哈希（Basic Auth 模式使用）'"
        ).await?;
        // 种子数据：CP001 密码 "test1234"
        db.execute_unprepared(&format!(
            "INSERT INTO charge_point_ocpp16 (charge_point_id, station_id, status, password_hash, is_deleted, create_time, update_time) \
             VALUES ('CP001', 1, 'Available', '{}', 0, NOW(), NOW()) \
             ON CONFLICT (charge_point_id) DO UPDATE SET password_hash = EXCLUDED.password_hash",
            SEED_CP001_HASH
        )).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE charge_point_ocpp16 DROP COLUMN IF EXISTS password_hash")
            .await?;
        Ok(())
    }
}
