use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tracing::{debug, info};

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> rdkafka::error::KafkaResult<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("compression.type", "lz4")
            .create()?;
        info!(brokers = %brokers, "Kafka 生产者已创建");
        Ok(Self { producer })
    }

    pub async fn send_resp(&self, topic: &str, key: &str, payload: &[u8]) -> Result<(), String> {
        let record = FutureRecord::to(topic).key(key).payload(payload);
        match self.producer.send(record, Duration::from_secs(5)).await {
            Ok((partition, offset)) => {
                debug!(
                    topic = %topic,
                    partition = %partition,
                    offset = %offset,
                    "Kafka 发送成功"
                );
                Ok(())
            }
            Err((e, _)) => Err(format!("Kafka 发送失败：{e}")),
        }
    }
}
