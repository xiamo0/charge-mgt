use crate::config::AppConfig;
use crate::error::AppError;
use crate::ocpp16::kafka::producer::KafkaProducer;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub config: Option<AppConfig>,
    pub db: Option<DatabaseConnection>,
    pub producer: Option<KafkaProducer>,
}

impl AppState {
    pub fn config(&self) -> Result<&AppConfig, AppError> {
        self.config
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("config".to_string()))
    }
    pub fn db(&self) -> Result<&DatabaseConnection, AppError> {
        self.db
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("db".to_string()))
    }
    pub fn producer(&self) -> Result<&KafkaProducer, AppError> {
        self.producer
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("producer".to_string()))
    }
}
