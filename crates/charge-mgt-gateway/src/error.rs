//! 网关统一错误类型

use thiserror::Error;

/// 网关运行时可能出现的各类错误
#[derive(Error, Debug)]
pub enum GatewayError {
    /// WebSocket 连接或通信错误
    #[error("WebSocket 错误: {0}")]
    WebSocket(String),

    /// 充电桩连接管理错误
    #[error("连接错误: {0}")]
    Connection(String),

    /// OCPP 协议层错误
    #[error("协议错误: {0}")]
    Protocol(String),

    /// 未识别的充电桩厂商
    #[error("未知厂商: {0}")]
    UnknownVendor(String),

    /// 消息序列化/反序列化错误
    #[error("消息编解码错误: {0}")]
    Codec(String),

    /// Kafka 生产/消费错误
    #[error("Kafka 错误: {0}")]
    Kafka(String),

    /// Redis 连接或命令错误
    #[error("Redis 错误: {0}")]
    Redis(String),

    /// 云端 REST API 调用错误
    #[error("云端 API 错误: {0}")]
    CloudApi(String),

    /// 配置加载或校验错误
    #[error("配置错误: {0}")]
    Config(String),

    /// 等待云端响应超时
    #[error("超时: {0}")]
    Timeout(String),

    /// TLS 握手或配置加载错误
    #[error("TLS 错误: {0}")]
    Tls(String),

    /// OCPP 桩身份验证失败
    #[error("认证错误: {0}")]
    Auth(String),
}

/// 网关操作结果类型别名
pub type Result<T> = std::result::Result<T, GatewayError>;
