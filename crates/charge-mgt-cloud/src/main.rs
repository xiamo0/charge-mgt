use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::Extension;
use axum::response::Json;
use axum::{Router, Server, routing::get};
use sea_orm::ConnectionTrait;
use tracing::info;

use charge_mgt_cloud::config::AppConfig;
use charge_mgt_cloud::infra::db;
use charge_mgt_cloud::ocpp16::kafka::consumer::spawn_kafka_consumer;
use charge_mgt_cloud::ocpp16::kafka::producer::KafkaProducer;
use charge_mgt_cloud::router;
use charge_mgt_cloud::state::AppState;

fn parse_config_path() -> PathBuf {
    std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .map(PathBuf::from)
        .or_else(|| std::env::var("CLOUD_CONFIG").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("crates/charge-mgt-cloud/config/default.yaml"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config_path = parse_config_path();
    let config = AppConfig::load(&config_path)?;

    info!(config_path = %config_path.display(), "已加载配置");

    let db = db::connect(&config.database.url, config.database.max_connections).await?;
    info!(url = %mask_url(&config.database.url), "已连接 PostgreSQL");

    db::run_migrations(&db).await?;
    info!("已应用数据库迁移");

    let state = AppState {
        config: Some(config.clone()),
        db: Some(db),
        producer: None,
    };
    #[cfg(feature = "send_message_by_mq")]
    {
        let producer = KafkaProducer::new(&config.kafka.brokers)?;
        state.producer = Some(producer);
    }

    spawn_kafka_consumer(state.clone()).await?;

    let app = build_router(state);

    let addr: std::net::SocketAddr = format!(
        "{}:{}",
        config.cloud.http_listen_addr, config.cloud.http_listen_port
    )
    .parse()?;

    info!(cloud_id = %config.cloud.id, %addr, "启动 charge-mgt-cloud HTTP 服务");

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

fn build_router(state: AppState) -> Router {
    let state = Arc::new(state);
    router::build()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(Extension(state))
}

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "charge-mgt-cloud",
        "description": "OCPP Charging Station Management System"
    }))
}

async fn health(Extension(state): Extension<Arc<AppState>>) -> Json<serde_json::Value> {
    let cloud_id = state
        .config()
        .map(|c| c.cloud.id.clone())
        .unwrap_or("unknown".into());
    if let Ok(db) = state.db() {
        let db_ok = db.execute_unprepared("SELECT 1").await.is_ok();
        let status = if db_ok { "ok" } else { "degraded" };
        Json(serde_json::json!({
            "status": status,
            "service": "charge-mgt-cloud",
            "version": env!("CARGO_PKG_VERSION"),
            "cloud_id": cloud_id,
            "components": {
                "database": db_ok,
                "kafka_producer": true
            }
        }))
    } else {
        Json(serde_json::json!({
            "status": "degraded",
            "service": "charge-mgt-cloud",
            "version": env!("CARGO_PKG_VERSION"),
            "cloud_id": cloud_id,
            "components": {
                "database": "not connected",
                "kafka_producer": false
            }
        }))
    }
}

fn mask_url(url: &str) -> String {
    let Some(at) = url.find("://") else {
        return "***".to_string();
    };
    let rest = &url[at + 3..];
    let Some(host) = rest.find('@') else {
        return url.to_string();
    };
    let masked = "****:****@";
    format!("{}://{}{}", &url[..at], masked, &rest[host + 1..])
}
