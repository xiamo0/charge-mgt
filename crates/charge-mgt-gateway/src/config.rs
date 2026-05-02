use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub gateway: GatewayConfig,
    pub device: DeviceConfig,
    pub cloud: CloudConfig,
    pub kafka: KafkaConfig,
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

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic_prefix: String,
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
            "kafka": {
                "brokers": "localhost:9092",
                "topic_prefix": "charge_mgt"
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.gateway.id, "gateway-01");
        assert_eq!(config.device.listen_port, 9000);
        assert_eq!(config.cloud.api_url, "https://cloud.example.com");
        assert_eq!(config.kafka.brokers, "localhost:9092");
    }
}
