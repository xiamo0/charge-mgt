use crate::config::AppConfig;
use crate::error::AppError;
#[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
use crate::ocpp16::kafka::producer::KafkaProducer;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub config: Option<AppConfig>,
    pub db: Option<DatabaseConnection>,
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub ocpp_1_6_producer: Option<KafkaProducer>,
}

impl AppState {
    pub fn new(config: AppConfig, db: DatabaseConnection) -> Self {
        Self {
            config: Some(config),
            db: Some(db),
            #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
            ocpp_1_6_producer: None, // 后续通过 with_producer() 设置
        }
    }

    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn with_producer(mut self, producer: KafkaProducer) -> Self {
        self.ocpp_1_6_producer = Some(producer);
        self
    }

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
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn producer(&self) -> Result<&KafkaProducer, AppError> {
        self.ocpp_1_6_producer
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("producer".to_string()))
    }
}
