use crate::config::AppConfig;
use crate::infra::db::DbPool;
use crate::infra::kafka::producer::KafkaProducer;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DbPool,
    pub producer: KafkaProducer,
}
