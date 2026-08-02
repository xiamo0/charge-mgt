use std::sync::Arc;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::infra::http_client::HttpSender;
#[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
use crate::ocpp16::kafka::mq_dispatcher::MqDispatcher;
#[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
use crate::ocpp16::kafka::producer::KafkaProducer;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub config: Option<AppConfig>,
    pub db: Option<DatabaseConnection>,
    pub http_sender: Option<HttpSender>,
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub ocpp_1_6_producer: Option<KafkaProducer>,
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub mq_dispatcher: Option<Arc<MqDispatcher>>,
}

impl AppState {
    pub fn new(config: AppConfig, db: DatabaseConnection, http_sender: HttpSender) -> Self {
        Self {
            config: Some(config),
            db: Some(db),
            http_sender: Some(http_sender),
            #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
            ocpp_1_6_producer: None,
            #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
            mq_dispatcher: None,
        }
    }

    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn with_producer(mut self, producer: KafkaProducer) -> Self {
        self.ocpp_1_6_producer = Some(producer);
        self
    }

    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn with_mq_dispatcher(mut self, dispatcher: Arc<MqDispatcher>) -> Self {
        self.mq_dispatcher = Some(dispatcher);
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
    pub fn http_sender(&self) -> Result<&HttpSender, AppError> {
        self.http_sender
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("http_sender".to_string()))
    }
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn producer(&self) -> Result<&KafkaProducer, AppError> {
        self.ocpp_1_6_producer
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("producer".to_string()))
    }
    #[cfg(all(feature = "ocpp_1_6", feature = "message_by_mq"))]
    pub fn mq_dispatcher(&self) -> Result<&Arc<MqDispatcher>, AppError> {
        self.mq_dispatcher
            .as_ref()
            .ok_or(AppError::ConfigNotInitialized("mq_dispatcher".to_string()))
    }
}
