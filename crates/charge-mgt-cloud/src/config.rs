use config::ConfigError;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub cloud: CloudConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    pub id: String,
    pub http_listen_addr: String,
    pub http_listen_port: u16,
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
