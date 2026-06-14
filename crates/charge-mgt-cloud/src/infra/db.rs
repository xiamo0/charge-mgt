use migration::MigratorTrait;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr};

pub async fn connect(url: &str, max_conns: u32) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(max_conns);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await?;
    db.execute_unprepared("SET statement_timeout = '30s'").await?;
    Ok(db)
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    migration::Migrator::up(db, None).await.map(|_| ())
}
