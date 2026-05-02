use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Unknown vendor: {0}")]
    UnknownVendor(String),

    #[error("Message encode/decode error: {0}")]
    Codec(String),

    #[error("Kafka error: {0}")]
    Kafka(String),

    #[error("Cloud API error: {0}")]
    CloudApi(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

pub type Result<T> = std::result::Result<T, GatewayError>;
