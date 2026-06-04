use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub gateway: GatewayConfig,
    pub device: DeviceConfig,
    pub cloud: CloudConfig,
    pub response_channel: ResponseChannelMode,
    #[serde(default)]
    pub redis: RedisConfig,
    pub kafka: KafkaConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ResponseChannelMode {
    #[default]
    Redis,
    Kafka,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    pub id: String,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloudConfig {
    pub api_url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RedisConfig {
    #[serde(default = "default_redis_url")]
    pub url: String,
    #[serde(default = "default_response_timeout")]
    pub response_timeout_secs: u64,
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

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic_prefix: String,
    #[serde(default = "default_req_suffix")]
    pub req_topic_suffix: String,
    #[serde(default = "default_resp_suffix")]
    pub resp_topic_suffix: String,
    #[serde(default = "default_cmd_suffix")]
    pub cmd_topic_suffix: String,
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
