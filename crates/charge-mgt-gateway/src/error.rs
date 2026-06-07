//! 网关统一错误类型

use thiserror::Error;

/// 网关运行时可能出现的各类错误
#[derive(Error, Debug)]
pub enum GatewayError {
    /// WebSocket 连接或通信错误
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// 充电桩连接管理错误
    #[error("Connection error: {0}")]
    Connection(String),

    /// OCPP 协议层错误
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// 未识别的充电桩厂商
    #[error("Unknown vendor: {0}")]
    UnknownVendor(String),

    /// 消息序列化/反序列化错误
    #[error("Message encode/decode error: {0}")]
    Codec(String),

    /// Kafka 生产/消费错误
    #[error("Kafka error: {0}")]
    Kafka(String),

    /// Redis 连接或命令错误
    #[error("Redis error: {0}")]
    Redis(String),

    /// 云端 REST API 调用错误
    #[error("Cloud API error: {0}")]
    CloudApi(String),

    /// 配置加载或校验错误
    #[error("Configuration error: {0}")]
    Config(String),

    /// 等待云端响应超时
    #[error("Timeout: {0}")]
    Timeout(String),
}

/// 网关操作结果类型别名
pub type Result<T> = std::result::Result<T, GatewayError>;
