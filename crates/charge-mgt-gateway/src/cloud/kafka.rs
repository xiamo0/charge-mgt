use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use tracing::{error, info};

use crate::cloud::message::CloudMessage;
use crate::config::KafkaConfig;
use crate::error::{GatewayError, Result};

pub struct KafkaProducer {
    producer: FutureProducer,
    topic_prefix: String,
}

impl KafkaProducer {
    pub async fn new(config: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", "5000")
            .set("queue.buffering.max.ms", "100")
            .set("acks", "1")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("Failed to create producer: {}", e)))?;

        info!("Kafka producer connected to {}", config.brokers);
        Ok(Self {
            producer,
            topic_prefix: config.topic_prefix.clone(),
        })
    }

    pub async fn send(&self, msg: &CloudMessage) -> Result<()> {
        let topic = format!("{}.{}", self.topic_prefix, msg.topic());
        let payload = serde_json::to_string(msg)
            .map_err(|e| GatewayError::Codec(format!("Failed to serialize message: {}", e)))?;

        let record = rdkafka::producer::FutureRecord::to(&topic)
            .payload(&payload)
            .key(&msg.charge_point_id);

        self.producer
            .send(record, std::time::Duration::from_secs(5))
            .await
            .map_err(|(e, _)| {
                error!("Failed to send message to Kafka: {}", e);
                GatewayError::Kafka(format!("Send failed: {}", e))
            })?;

        info!(
            "[KAFKA] Message sent to topic={}, key={}",
            topic, msg.charge_point_id
        );
        Ok(())
    }
}

pub struct MockKafkaProducer {
    topic_prefix: String,
}

impl MockKafkaProducer {
    pub fn new(config: &KafkaConfig) -> Self {
        info!(
            "Mock Kafka producer created, prefix: {}",
            config.topic_prefix
        );
        Self {
            topic_prefix: config.topic_prefix.clone(),
        }
    }

    pub async fn send(&self, msg: &CloudMessage) -> Result<()> {
        let topic = format!("{}.{}", self.topic_prefix, msg.topic());
        let payload = serde_json::to_string(msg)
            .map_err(|e| GatewayError::Codec(format!("Failed to serialize: {}", e)))?;

        info!(
            "[MOCK] Message would be sent to Kafka, topic={}, payload_len={}",
            topic,
            payload.len()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cloud::{CloudMessage, CloudMessageInput, MockKafkaProducer};

    #[tokio::test]
    async fn test_mock_kafka_send() {
        let config = crate::config::KafkaConfig {
            brokers: "localhost:9092".to_string(),
            topic_prefix: "test".to_string(),
        };

        let producer = MockKafkaProducer::new(&config);

        let input = CloudMessageInput {
            gateway_id: "gateway-01".to_string(),
            gateway_ip: "192.168.1.100".to_string(),
            vendor: "Alphas".to_string(),
            charge_point_id: "CB001".to_string(),
            protocol: "OCPP-1.6".to_string(),
            message_type: "Call".to_string(),
            action: "BootNotification".to_string(),
            unique_id: "uuid-001".to_string(),
        };

        let msg = CloudMessage::new(input, serde_json::json!({}));

        let result = producer.send(&msg).await;
        assert!(result.is_ok());
    }
}
