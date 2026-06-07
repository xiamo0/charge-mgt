//! WebSocket 服务器
//!
//! 监听 TCP 端口，接受充电桩 WebSocket 连接，
//! 读写分离：读任务处理上行消息，写任务发送响应。

use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

use crate::cloud::{ConnectionManager, KafkaProducer};
use crate::config::DeviceConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;

/// WebSocket 服务端，管理充电桩接入
pub struct WebSocketServer {
    /// WebSocket 监听配置
    config: DeviceConfig,
    /// 在线充电桩连接注册表
    connection_manager: Arc<ConnectionManager>,
    /// Kafka 消息生产者
    kafka_producer: Arc<KafkaProducer>,
    /// 云端响应回传通道
    response_channel: Arc<dyn ResponseChannel>,
    /// 所属网关 ID
    gateway_id: String,
    /// 所属网关 IP
    gateway_host: String,
}

impl WebSocketServer {
    /// 创建 WebSocket 服务器实例
    pub fn new(
        config: DeviceConfig,
        connection_manager: Arc<ConnectionManager>,
        kafka_producer: Arc<KafkaProducer>,
        response_channel: Arc<dyn ResponseChannel>,
        gateway_id: String,
        gateway_host: String,
    ) -> Self {
        Self {
            config,
            connection_manager,
            kafka_producer,
            response_channel,
            gateway_id,
            gateway_host,
        }
    }

    /// 绑定监听地址，循环接受连接并为每个连接 spawn 独立任务
    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.listen_port);
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| GatewayError::Config(format!("Invalid address: {}", e)))?;

        info!("WebSocket 服务正在启动，地址: {}", addr);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| GatewayError::WebSocket(format!("Failed to bind: {}", e)))?;

        info!("WebSocket 服务已监听: {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    info!("新连接来自: {}", peer_addr);
                    let connection_manager = self.connection_manager.clone();
                    let kafka_producer = self.kafka_producer.clone();
                    let response_channel = self.response_channel.clone();
                    let gateway_id = self.gateway_id.clone();
                    let gateway_host = self.gateway_host.clone();
                    tokio::spawn(handle_connection(
                        stream,
                        peer_addr,
                        connection_manager,
                        kafka_producer,
                        response_channel,
                        gateway_id,
                        gateway_host,
                    ));
                }
                Err(e) => {
                    error!("接受连接失败: {}", e);
                }
            }
        }
    }
}

/// 处理单条 WebSocket 连接的生命周期（握手 → 读写 → 清理）
async fn handle_connection(
    stream: tokio::net::TcpStream,
    peer_addr: SocketAddr,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    response_channel: Arc<dyn ResponseChannel>,
    gateway_id: String,
    gateway_host: String,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            warn!("WebSocket 握手失败，来自 {}: {}", peer_addr, e);
            return;
        }
    };

    let (ws_write, ws_read) = ws_stream.split();

    let (response_tx, mut response_rx) = mpsc::unbounded_channel();

    let mut connection = crate::device::connection::Connection::new(
        peer_addr,
        connection_manager.clone(),
        kafka_producer,
        response_channel,
        gateway_id,
        gateway_host,
        response_tx.clone(),
    );

    let mut ws_read = ws_read;
    let mut ws_write = ws_write;

    let read_task = async {
        use tokio_tungstenite::tungstenite::Message;

        while let Some(msg) = ws_read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("收到消息，来自 {}: {}", peer_addr, text);
                    match connection.handle_message(&text).await {
                        Ok(()) => {}
                        Err(e) => {
                            error!("处理消息失败，来自 {}: {}", peer_addr, e);
                            let error_response =
                                connection.create_call_error("", "InternalError", &e.to_string());
                            response_tx.send(error_response).ok();
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("连接已关闭: {}", peer_addr);
                    connection.on_disconnect().await;
                    break;
                }
                Err(e) => {
                    warn!("读取消息失败，来自 {}: {}", peer_addr, e);
                    connection.on_disconnect().await;
                    break;
                }
                _ => {}
            }
        }
    };

    let write_task = async {
        use tokio_tungstenite::tungstenite::Message;

        while let Some(response) = response_rx.recv().await {
            if ws_write.send(Message::Text(response)).await.is_err() {
                warn!("发送消息失败，目标 {}: WebSocket 写入错误", peer_addr);
                break;
            }
        }
    };

    tokio::select! {
        _ = read_task => {},
        _ = write_task => {},
    }

    connection.on_disconnect().await;
}
