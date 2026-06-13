use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Executor;

pub type DbPool = PgPool;

pub async fn connect(url: &str, max_conns: u32) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_conns)
        .connect(url)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    pool.execute("SET statement_timeout = '30s'")
        .await
        .map_err(sqlx::migrate::MigrateError::Execute)?;
    let migrator = sqlx::migrate!("./migrations");
    migrator.run(pool).await?;
    Ok(())
}
