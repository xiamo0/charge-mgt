use crate::config::AppConfig;
use crate::infra::kafka::producer::KafkaProducer;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,
    pub producer: KafkaProducer,
}
