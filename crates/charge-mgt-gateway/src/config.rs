//! 网关配置定义与加载
//!
//! 通过 `CONFIG_PATH` 环境变量指定配置文件路径，默认为 `config/default`。

use serde::Deserialize;
use std::path::PathBuf;

/// 网关完整配置
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 网关自身标识配置
    pub gateway: GatewayConfig,
    /// WebSocket 监听配置
    pub device: DeviceConfig,
    /// 云端 REST API 配置
    pub cloud: CloudConfig,
    /// 云端响应回传方式
    pub response_channel: ResponseChannelMode,
    /// Redis 配置（response_channel 为 redis 时使用）
    #[serde(default)]
    pub redis: RedisConfig,
    /// Kafka 消息队列配置
    pub kafka: KafkaConfig,
    /// OCPP 1.6 链路安全配置（TLS + Basic Auth 组合）
    #[serde(default)]
    pub ocpp_security: OcppSecurityConfig,
}

/// OCPP 1.6 桩→gateway 链路的安全配置
///
/// 6 种模式（OCPP 1.6 Profile 1/2/3/3+Basic/3+mTLS/3+mTLS+Basic）：
/// | auth_mode    | tls.enabled | tls.mtls | 派生模式 | 适用 |
/// |--------------|-------------|----------|----------|------|
/// | none         | false       | -        | 模式 1：明文无认证 | 内部/调试 |
/// | none         | true        | -        | 模式 3：TLS 无认证 | 加密但无身份 |
/// | basic        | false       | -        | 模式 2：明文 Basic | 内网+密码（过渡） |
/// | basic        | true        | -        | 模式 4：TLS + Basic | 主流生产 |
/// | mtls         | true        | enabled  | 模式 5：TLS + mTLS | 高安全 |
/// | mtls-with-basic | true     | enabled  | 模式 6：TLS + mTLS + Basic | 最高安全 |
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OcppSecurityConfig {
    /// 认证模式
    pub auth_mode: AuthMode,
    /// TLS 配置
    #[serde(default)]
    pub tls: TlsConfig,
}

/// 认证模式
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuthMode {
    /// 不要求任何认证（模式 1 / 模式 3）
    #[default]
    None,
    /// HTTP Basic Auth（模式 2 / 模式 4）
    Basic,
    /// 仅 mTLS（模式 5；不能 basic 模式）
    Mtls,
    /// mTLS + Basic Auth（模式 6）
    MtlsWithBasic,
}

/// TLS 配置
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TlsConfig {
    /// 是否启用 TLS（false = 明文 ws://；true = 加密 wss://）
    #[serde(default)]
    pub enabled: bool,
    /// 服务端证书 PEM 路径
    pub cert_path: Option<PathBuf>,
    /// 服务端私钥 PEM 路径
    pub key_path: Option<PathBuf>,
    /// mTLS 配置（桩客户端证书校验；模式 5/6 必填）
    #[serde(default)]
    pub mtls: Option<MtlsConfig>,
}

/// mTLS 配置：CSMS 用 CA 证书验证桩客户端证书
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MtlsConfig {
    /// CA 证书 PEM 路径（用于构造 Client Cert Verifier）
    pub ca_cert_path: PathBuf,
    /// 客户端证书校验模式：required（拒所有无证书的连接）或 optional（允许无证书但有证书时也验）
    #[serde(default = "default_required")]
    pub client_auth: ClientAuthMode,
}

fn default_required() -> ClientAuthMode {
    ClientAuthMode::Required
}

/// 客户端证书校验模式
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ClientAuthMode {
    /// 要求所有客户端必须提供证书（mTLS 强制）
    Required,
    /// 可选：允许无证书的连接（一般用于同时支持 mTLS 和匿名 TLS 的过渡期）
    Optional,
}

impl Default for ClientAuthMode {
    fn default() -> Self {
        Self::Required
    }
}

/// 云端响应回传方式：Redis BLPOP 或 Kafka 响应主题
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResponseChannelMode {
    #[default]
    Redis,
    Kafka,
}

/// 网关自身标识，用于 Kafka 主题路由
#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    /// 网关唯一 ID，用于 Kafka 主题路由
    pub id: String,
    /// 网关对外 IP 地址，写入上行消息元数据
    pub host: String,
}

/// WebSocket 监听地址配置
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceConfig {
    /// WebSocket 监听地址（如 `0.0.0.0`）
    pub listen_addr: String,
    /// WebSocket 监听端口
    pub listen_port: u16,
}

/// 云端 REST API 配置
#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    /// 云端 API 基础 URL
    pub api_url: String,
    /// API 认证密钥（Bearer Token）
    pub api_key: String,
}

/// Redis 响应通道配置
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RedisConfig {
    /// Redis 连接地址
    #[serde(default = "default_redis_url")]
    pub url: String,
    /// BLPOP 等待云端响应的超时时间（秒）
    #[serde(default = "default_response_timeout")]
    pub response_timeout_secs: u64,
    /// 响应键的 TTL（秒）
    #[serde(default = "default_key_ttl")]
    pub key_ttl_secs: u64,
}

fn default_redis_url() -> String {
    "redis://127.0.0.1:6379".to_string()
}

fn default_response_timeout() -> u64 {
    5
}

fn default_key_ttl() -> u64 {
    10
}

/// Kafka 消息队列配置
#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    /// Kafka broker 地址列表（逗号分隔）
    pub brokers: String,
    /// 主题名前缀（如 `charge_mgt`）
    pub topic_prefix: String,
    /// 上行请求主题后缀，完整主题：`{prefix}.{req_suffix}.{vendor}`
    #[serde(default = "default_req_suffix")]
    pub req_topic_suffix: String,
    /// 响应主题后缀（Kafka 响应通道模式）
    #[serde(default = "default_resp_suffix")]
    pub resp_topic_suffix: String,
    /// 命令主题后缀（Redis 响应通道模式）
    #[serde(default = "default_cmd_suffix")]
    pub cmd_topic_suffix: String,
    /// 待响应请求超时时间（秒，Kafka 响应通道模式）
    #[serde(default = "default_kafka_timeout")]
    pub response_timeout_secs: u64,
}

fn default_req_suffix() -> String {
    "req".to_string()
}

fn default_resp_suffix() -> String {
    "resp".to_string()
}

fn default_cmd_suffix() -> String {
    "cmd".to_string()
}

fn default_kafka_timeout() -> u64 {
    30
}

impl Config {
    /// 从配置文件加载配置，路径由 `CONFIG_PATH` 环境变量决定
    pub fn load() -> Result<Self, config::ConfigError> {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default".to_string());

        let settings = config::Config::builder()
            .add_source(config::File::with_name(&config_path).required(true))
            .build()?;

        settings.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        let json = r#"{
            "gateway": {
                "id": "gateway-01",
                "host": "192.168.1.100"
            },
            "device": {
                "listen_addr": "0.0.0.0",
                "listen_port": 9000
            },
            "cloud": {
                "api_url": "https://cloud.example.com",
                "api_key": "test_key"
            },
            "response_channel": "redis",
            "redis": {
                "url": "redis://127.0.0.1:6379",
                "response_timeout_secs": 5,
                "key_ttl_secs": 10
            },
            "kafka": {
                "brokers": "localhost:9092",
                "topic_prefix": "charge_mgt",
                "req_topic_suffix": "req",
                "resp_topic_suffix": "resp",
                "cmd_topic_suffix": "cmd",
                "response_timeout_secs": 30
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.gateway.id, "gateway-01");
        assert_eq!(config.device.listen_port, 9000);
        assert_eq!(config.cloud.api_url, "https://cloud.example.com");
        assert_eq!(config.kafka.brokers, "localhost:9092");
        assert_eq!(config.response_channel, ResponseChannelMode::Redis);
        assert_eq!(config.redis.response_timeout_secs, 5);
        assert_eq!(config.kafka.req_topic_suffix, "req");
        assert_eq!(config.kafka.resp_topic_suffix, "resp");
    }

    #[test]
    fn test_config_kafka_mode() {
        let json = r#"{
            "gateway": {"id": "gw-02", "host": "10.0.0.1"},
            "device": {"listen_addr": "0.0.0.0", "listen_port": 9000},
            "cloud": {"api_url": "https://cloud.example.com", "api_key": "key"},
            "response_channel": "kafka",
            "kafka": {"brokers": "localhost:9092", "topic_prefix": "charge_mgt"}
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.response_channel, ResponseChannelMode::Kafka);
        assert_eq!(config.kafka.req_topic_suffix, "req"); // default
    }
}
