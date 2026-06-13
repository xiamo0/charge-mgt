pub mod db;
pub mod kafka;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("kafka producer error: {0}")]
    KafkaProducer(String),

    #[error("kafka consumer error: {0}")]
    KafkaConsumer(String),

    #[error("kafka client create error: {0}")]
    KafkaClient(#[from] rdkafka::error::KafkaError),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}
