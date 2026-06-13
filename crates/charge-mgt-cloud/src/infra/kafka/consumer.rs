use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tracing::{error, info, warn};

use crate::ocpp::dispatcher::MessageDispatcher;
use crate::state::AppState;

pub async fn spawn_kafka_consumer(state: AppState) -> anyhow::Result<()> {
    let topics: Vec<String> = state.config.kafka.req_topics.clone();
    if topics.is_empty() {
        warn!("no req_topics configured, skipping kafka consumer");
        return Ok(());
    }

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &state.config.kafka.consumer_group)
        .set("bootstrap.servers", &state.config.kafka.brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "10000")
        .create()?;

    let topic_refs: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
    consumer.subscribe(&topic_refs)?;

    info!(consumer_group = %state.config.kafka.consumer_group, "Kafka consumer subscribed to topics: {:?}", topics);

    let dispatcher = MessageDispatcher::new(state);

    tokio::spawn(async move {
        let mut stream = consumer.stream();
        loop {
            match stream.next().await {
                Some(Ok(borrowed_message)) => {
                    if let Some(payload) = borrowed_message.payload() {
                        if let Err(e) = dispatcher.dispatch(payload).await {
                            error!(error = %e, "dispatch failed");
                        }
                    }
                }
                Some(Err(e)) => {
                    error!(error = %e, "kafka consume error");
                }
                None => {
                    warn!("kafka stream ended");
                    break;
                }
            }
        }
    });

    Ok(())
}
