use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};
use futures_util::{SinkExt, StreamExt};

use crate::cloud::{ConnectionManager, KafkaProducer};
use crate::config::DeviceConfig;
use crate::error::{GatewayError, Result};
use crate::response_channel::ResponseChannel;

pub struct WebSocketServer {
    config: DeviceConfig,
    connection_manager: Arc<ConnectionManager>,
    kafka_producer: Arc<KafkaProducer>,
    response_channel: Arc<dyn ResponseChannel>,
    gateway_id: String,
    gateway_host: String,
}

impl WebSocketServer {
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

    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.listen_port);
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| GatewayError::Config(format!("Invalid address: {}", e)))?;

        info!("WebSocket server starting on {}", addr);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| GatewayError::WebSocket(format!("Failed to bind: {}", e)))?;

        info!("WebSocket server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    info!("New connection from: {}", peer_addr);
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
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

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
            warn!("WebSocket handshake failed from {}: {}", peer_addr, e);
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
                    info!("Received from {}: {}", peer_addr, text);
                    match connection.handle_message(&text).await {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Error handling message from {}: {}", peer_addr, e);
                            let error_response = connection.create_call_error(
                                "",
                                "InternalError",
                                &e.to_string(),
                            );
                            response_tx.send(error_response).ok();
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("Connection closed: {}", peer_addr);
                    connection.on_disconnect().await;
                    break;
                }
                Err(e) => {
                    warn!("Error reading from {}: {}", peer_addr, e);
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
                warn!("Failed to send to {}: WebSocket write error", peer_addr);
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