use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::ocpp16::dispatcher::MessageDispatcher;
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
        warn!("无可用 req 话题，消费者空闲（Gateway 启动后重启 cloud）");
        return Ok(());
    }

    let topic_refs: Vec<&str> = topics.iter().map(|s| s.as_str()).collect();
    consumer.subscribe(&topic_refs)?;

    info!(
        consumer_group = %state.config.kafka.consumer_group,
        "Kafka 消费者已订阅 {} 个话题：{:?}",
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
                                    error!(error = %e, "分发失败");
                                }
                            }
                        }
                        Some(Err(e)) => {
                            error!(error = %e, "Kafka 消费错误");
                        }
                        None => {
                            warn!("Kafka 流已结束");
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
                            warn!("重发现 metadata 拉取失败：{e}");
                            continue;
                        }
                    };
                    let discovered: Vec<String> = metadata.topics().iter()
                        .map(|t| t.name().to_string())
                        .filter(|n| n.starts_with(&pattern))
                        .collect();
                    if discovered.len() > topic_count {
                        info!(
                            "重发现：{} -> {} 个话题，重新订阅",
                            topic_count,
                            discovered.len(),
                        );
                        let refs: Vec<&str> = discovered.iter().map(|s| s.as_str()).collect();
                        match consumer.subscribe(&refs) {
                            Ok(()) => {
                                topic_count = discovered.len();
                                stream = consumer.stream();
                                info!("已重新订阅 {} 个话题", topic_count);
                            }
                            Err(e) => error!("重新订阅失败：{e}"),
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
        info!("使用配置中显式指定的 req_topics：{:?}", topics);
        return topics.clone();
    }

    let pattern = format!("{}.req.", state.config.kafka.topic_prefix);
    let metadata = match consumer.fetch_metadata(None, Duration::from_secs(10)) {
        Ok(m) => m,
        Err(e) => {
            warn!("拉取 Kafka metadata 失败：{e}");
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
        warn!("未找到匹配 '{}*' 的话题——Gateway 是否已启动？", pattern,);
    } else {
        info!(
            "已发现 {} 个匹配 `{}*` 的 req 话题：{:?}",
            discovered.len(),
            pattern,
            discovered,
        );
    }

    discovered
}
