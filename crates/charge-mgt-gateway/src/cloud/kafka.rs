//! Kafka 消息生产者
//!
//! 将充电桩上行消息发布到按厂商分区的请求主题。

use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use tracing::{error, info};

use crate::cloud::message::CloudMessage;
use crate::config::KafkaConfig;
use crate::error::{GatewayError, Result};

/// Kafka 生产者，将 CloudMessage 发布到请求主题
pub struct KafkaProducer {
    /// rdkafka 异步生产者
    producer: FutureProducer,
    /// 主题名前缀
    topic_prefix: String,
    /// 请求主题后缀
    req_topic_suffix: String,
}

impl KafkaProducer {
    /// 连接 Kafka 集群并创建生产者
    pub async fn new(config: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", "5000")
            .set("queue.buffering.max.ms", "100")
            .set("acks", "1")
            .create()
            .map_err(|e| GatewayError::Kafka(format!("Failed to create producer: {}", e)))?;

        info!("Kafka 生产者已连接: {}", config.brokers);
        Ok(Self {
            producer,
            topic_prefix: config.topic_prefix.clone(),
            req_topic_suffix: config.req_topic_suffix.clone(),
        })
    }

    /// 将消息序列化后发送到 `{prefix}.{req_suffix}.{vendor}` 主题
    pub async fn send(&self, msg: &CloudMessage) -> Result<()> {
        let topic = msg.req_topic(&self.topic_prefix, &self.req_topic_suffix);
        let payload = serde_json::to_string(msg)
            .map_err(|e| GatewayError::Codec(format!("Failed to serialize message: {}", e)))?;

        let record = rdkafka::producer::FutureRecord::to(&topic)
            .payload(&payload)
            .key(&msg.charge_point_id);

        self.producer
            .send(record, std::time::Duration::from_secs(5))
            .await
            .map_err(|(e, _)| {
                error!("Kafka 消息发送失败: {}", e);
                GatewayError::Kafka(format!("Send failed: {}", e))
            })?;

        info!(
            "[KAFKA] 消息已发送，主题={}, 键={}",
            topic, msg.charge_point_id
        );
        Ok(())
    }
}

/// 模拟 Kafka 生产者，仅记录日志不实际发送（用于测试）
pub struct MockKafkaProducer {
    /// 主题名前缀
    topic_prefix: String,
    /// 请求主题后缀
    req_topic_suffix: String,
}

impl MockKafkaProducer {
    /// 创建模拟生产者
    pub fn new(config: &KafkaConfig) -> Self {
        info!(
            "模拟 Kafka 生产者已创建，前缀: {}",
            config.topic_prefix
        );
        Self {
            topic_prefix: config.topic_prefix.clone(),
            req_topic_suffix: config.req_topic_suffix.clone(),
        }
    }

    /// 模拟发送，仅打印目标主题和 payload 长度
    pub async fn send(&self, msg: &CloudMessage) -> Result<()> {
        let topic = msg.req_topic(&self.topic_prefix, &self.req_topic_suffix);
        let payload = serde_json::to_string(msg)
            .map_err(|e| GatewayError::Codec(format!("Failed to serialize: {}", e)))?;

        info!(
            "[MOCK] 模拟发送 Kafka 消息，主题={}, 载荷长度={}",
            topic,
            payload.len()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cloud::{CloudMessage, CloudMessageInput, MockKafkaProducer};
    use crate::config::KafkaConfig;

    #[tokio::test]
    async fn test_mock_kafka_send() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            topic_prefix: "test".to_string(),
            req_topic_suffix: "req".to_string(),
            resp_topic_suffix: "resp".to_string(),
            cmd_topic_suffix: "cmd".to_string(),
            response_timeout_secs: 30,
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