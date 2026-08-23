use config::ConfigError;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub cloud: CloudConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// JWT 签名密钥（HS256）。生产必须通过环境变量 CLOUD__AUTH__JWT_SECRET 覆盖
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    /// access token 有效期（秒）
    #[serde(default = "default_ttl")]
    pub access_token_ttl_secs: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: default_jwt_secret(),
            access_token_ttl_secs: default_ttl(),
        }
    }
}

fn default_jwt_secret() -> String {
    "dev-jwt-secret-CHANGEME-0123456789".to_string()
}

fn default_ttl() -> u64 {
    3600
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    pub id: String,
    pub http_listen_addr: String,
    pub http_listen_port: u16,
    /// 内网 API 共享密钥（gateway 与 cloud 一致）。用于 /internal/* 路由的 Bearer 认证。
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_conns")]
    pub max_connections: u32,
}

fn default_max_conns() -> u32 {
    10
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    #[serde(default = "default_consumer_group")]
    pub consumer_group: String,
    #[serde(default = "default_topic_prefix")]
    pub topic_prefix: String,
    /// 需消费的 Kafka topic 列表；若为空则启动时自动发现所有 {topic_prefix}.req.* 话题
    #[serde(default)]
    pub req_topics: Vec<String>,
}

fn default_consumer_group() -> String {
    "charge-mgt-cloud-consumer".to_string()
}

fn default_topic_prefix() -> String {
    "charge_mgt".to_string()
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(PathBuf::from(path)))
            .add_source(config::Environment::with_prefix("CLOUD").separator("__"))
            .build()?;
        settings.try_deserialize()
    }
}
