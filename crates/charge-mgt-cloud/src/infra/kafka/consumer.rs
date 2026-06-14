use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::ocpp::dispatcher::MessageDispatcher;
use crate::state::AppState;

pub async fn spawn_kafka_consumer(state: AppState) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &state.config.kafka.consumer_group)
        .set("bootstrap.servers", &state.config.kafka.brokers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "10000")
        .create()?;

    let topics = resolve_req_topics(&state, &consumer);
    if topics.is_empty() {
        warn!("no req topics available, consumer will be idle (restart cloud when gateway is running)");
        return Ok(());
    }

    let topic_refs: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
    consumer.subscribe(&topic_refs)?;

    info!(
        consumer_group = %state.config.kafka.consumer_group,
        "Kafka consumer subscribed to {} topics: {:?}",
        topics.len(),
        topics,
    );

    let kafka_cfg = state.config.kafka.clone();
    let dispatcher = MessageDispatcher::new(state);
    let pattern = format!("{}.req.", kafka_cfg.topic_prefix);
    let re_discover = kafka_cfg.req_topics.is_empty();

    tokio::spawn(async move {
        let mut stream = consumer.stream();
        let mut topic_count = topics.len();
        let mut tick = tokio::time::interval(Duration::from_secs(60));
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                msg = stream.next() => {
                    match msg {
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
                _ = tick.tick() => {
                    if !re_discover {
                        continue;
                    }
                    let metadata = match consumer.fetch_metadata(None, Duration::from_secs(5)) {
                        Ok(m) => m,
                        Err(e) => {
                            warn!("re-discovery metadata fetch failed: {e}");
                            continue;
                        }
                    };
                    let discovered: Vec<String> = metadata.topics().iter()
                        .map(|t| t.name().to_string())
                        .filter(|n| n.starts_with(&pattern))
                        .collect();
                    if discovered.len() > topic_count {
                        info!(
                            "re-discovery: {} -> {} topics, re-subscribing",
                            topic_count,
                            discovered.len(),
                        );
                        let refs: Vec<&str> = discovered.iter().map(|s| s.as_str()).collect();
                        match consumer.subscribe(&refs) {
                            Ok(()) => {
                                topic_count = discovered.len();
                                stream = consumer.stream();
                                info!("re-subscribed to {} topics", topic_count);
                            }
                            Err(e) => error!("re-subscribe failed: {e}"),
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

fn resolve_req_topics(state: &AppState, consumer: &StreamConsumer) -> Vec<String> {
    let topics = &state.config.kafka.req_topics;
    if !topics.is_empty() {
        info!("using explicit req_topics from config: {:?}", topics);
        return topics.clone();
    }

    let pattern = format!("{}.req.", state.config.kafka.topic_prefix);
    let metadata = match consumer.fetch_metadata(None, Duration::from_secs(10)) {
        Ok(m) => m,
        Err(e) => {
            warn!("failed to fetch kafka metadata: {e}");
            return Vec::new();
        }
    };

    let discovered: Vec<String> = metadata
        .topics()
        .iter()
        .map(|t| t.name().to_string())
        .filter(|name| name.starts_with(&pattern))
        .collect();

    if discovered.is_empty() {
        warn!(
            "no topics found matching '{}*' — is the gateway running?",
            pattern,
        );
    } else {
        info!(
            "discovered {} req topics matching `{}*`: {:?}",
            discovered.len(),
            pattern,
            discovered,
        );
    }

    discovered
}
